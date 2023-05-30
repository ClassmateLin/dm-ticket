use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use crate::{
    client::DmClient,
    config::Account,
    models::{
        order::{OrderForm, OrderInfo, OrderParams, SubmitOrderParams},
        perform::{PerformForm, PerformInfo, PerformParams},
        ticket::{TicketInfo, TicketInfoForm, TicketInfoParams},
        DmRes,
    },
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use log::{debug, error, info, warn};
use serde_json::json;
use tokio::signal;

const SUCCESS_FLAG: &str = "SUCCESS::调用成功";

pub struct DmTicket {
    pub client: DmClient,
    pub account: Account,
}

impl DmTicket {
    pub async fn new(account: Account) -> Result<Self> {
        let cookie = account
            .cookie
            .clone()
            .replace(' ', "")
            .split(';')
            .filter(|e| !e.starts_with("_m_h5_tk"))
            .collect::<Vec<&str>>()
            .join(";");

        let client = DmClient::new(cookie).await?;

        Ok(Self { client, account })
    }

    // 获取门票信息
    pub async fn get_ticket_info(&self, ticket_id: String) -> Result<TicketInfo> {
        let url = "https://mtop.damai.cn/h5/mtop.alibaba.damai.detail.getdetail/1.2";

        let params = TicketInfoParams::build()?;

        let data = TicketInfoForm::build(ticket_id)?;

        let res = self.client.request(url, params, data).await?;

        match res.ret.contains(&SUCCESS_FLAG.to_string()) {
            true => {
                debug!("获取门票信息成功, {:?}", res);

                let ticket_info: TicketInfo =
                    serde_json::from_str(res.data["result"].clone().as_str().unwrap())?;
                Ok(ticket_info)
            }
            false => {
                error!("获取门票信息失败, 结果:{:?}", res.ret);
                Err(anyhow!("获取门票信息失败..."))
            }
        }
    }

    // 生成订单
    pub async fn build_order(&self, item_id: &String, sku_id: &String) -> Result<OrderInfo> {
        let start = Instant::now();

        let url = "https://mtop.damai.cn/h5/mtop.trade.order.build.h5/4.0/?";

        let params = OrderParams::build()?;

        let data = OrderForm::build(item_id, sku_id, self.account.ticket.num)?;

        let res = self.client.request(url, params, data).await?;

        debug!("生成订单结果:{:?}, 花费时间:{:?}", res, start.elapsed());

        match res.ret.contains(&SUCCESS_FLAG.to_string()) {
            true => {
                let order_info: OrderInfo = serde_json::from_value(res.data)?;
                Ok(order_info)
            }
            false => Err(anyhow!("{:?}", res.ret)),
        }
    }

    // 提交订单
    pub async fn submit_order(&self, order_info: OrderInfo) -> Result<DmRes> {
        let start = Instant::now();

        let url = "https://mtop.damai.cn/h5/mtop.trade.order.create.h5/4.0/";

        // 添加提交订单需要的数据
        let mut order_data = json!({});

        for key in order_info.linkage.input.iter() {
            if key.starts_with("dmViewer_") {
                let mut item = order_info.data[key].clone();
                let mut num = self.account.ticket.num;

                let viewer_list = item["fields"]["viewerList"].clone();

                // 需选择实名观演人
                if viewer_list.is_array() && !viewer_list.as_array().unwrap().is_empty() {
                    // 实名观演人比购票数量少
                    if viewer_list.as_array().unwrap().len() < num {
                        warn!("实名观演人小于实际购票数量, 请先添加实名观演人!");
                        num = viewer_list.as_array().unwrap().len();
                    }
                    for i in 0..num {
                        item["fields"]["viewerList"][i]["isUsed"] = true.into();
                    }
                }
                order_data[key] = item;
            } else {
                order_data[key] = order_info.data[key].clone();
            }
        }

        // 添加confirmOrder_1
        let confirm_order_key = &order_info.hierarchy.root;
        order_data[confirm_order_key] = order_info.data[confirm_order_key].clone();

        // 添加order_xxxxx
        let keys_list = order_info.hierarchy.structure[confirm_order_key].clone();
        for k in keys_list.as_array().unwrap() {
            let s = k.as_str().unwrap();
            if s.starts_with("order_") {
                order_data[s] = order_info.data[s].clone();
            }
        }

        let order_hierarchy = json!({
            "structure": order_info.hierarchy.structure
        });

        let order_linkage = json!({
            "common": {
                "compress": order_info.linkage.common.compress,
                "submitParams": order_info.linkage.common.submit_params,
                "validateParams": order_info.linkage.common.validate_params,
            },
            "signature": order_info.linkage.signature,
        });

        let submit_order_params = SubmitOrderParams::build(order_info.global.secret_value)?;

        let feature = json!({
            "subChannel": "damai@damaih5_h5",
            "returnUrl": "https://m.damai.cn/damai/pay-success/index.html?spm=a2o71.orderconfirm.bottom.dconfirm&sqm=dianying.h5.unknown.value",
            "serviceVersion": "2.0.0",
            "dataTags": "sqm:dianying.h5.unknown.value"
        });
        let params = json!({
            "data": serde_json::to_string(&order_data)?,
            "hierarchy": serde_json::to_string(&order_hierarchy)?,
            "linkage": serde_json::to_string(&order_linkage)?,
        });
        let sumbit_order_data = json!({
            "params": serde_json::to_string(&params)?,
            "feature": serde_json::to_string(&feature)?,
        });

        let res = self
            .client
            .request(url, submit_order_params, sumbit_order_data)
            .await?;

        debug!("提交订单结果:{:?}, 花费时间:{:?}", res, start.elapsed());
        Ok(res)
    }

    pub async fn get_perform_info(
        &self,
        ticket_id: String,
        perform_id: String,
    ) -> Result<PerformInfo> {
        let start = Instant::now();

        let url = "https://mtop.damai.cn/h5/mtop.alibaba.detail.subpage.getdetail/2.0/";

        let params = PerformParams::build()?;

        let data = PerformForm::build(ticket_id, perform_id)?;

        let res = self.client.request(url, params, data).await?;

        debug!("获取演出票档信息:{:?}, 花费时间:{:?}", res, start.elapsed());

        let perform_info: PerformInfo = serde_json::from_str(res.data["result"].as_str().unwrap())?;

        Ok(perform_info)
    }

    pub async fn buy(&self, item_id: &String, sku_id: &String) -> Result<bool> {
        let start = Instant::now();

        let order_info = match self.build_order(item_id, sku_id).await {
            Ok(data) => {
                info!("成功生成订单...");
                data
            }
            Err(e) => {
                info!("生成订单失败, {}", e);
                return Ok(false);
            }
        };

        let res = self.submit_order(order_info).await?;

        match res.ret.contains(&SUCCESS_FLAG.to_string()) {
            true => {
                info!(
                    "提交订单成功, 请尽快前往手机APP付款,  此次抢购花费时间:{:?}",
                    start.elapsed()
                );
                Ok(true)
            }
            false => {
                info!(
                    "提交订单失败, 原因:{}, 此次抢购花费时间:{:?}",
                    res.ret[0],
                    start.elapsed()
                );
                Ok(false)
            }
        }
    }

    pub fn ms_to_hms(&self, ms: i64) -> (u64, u64, f64) {
        let sec = ms as f64 / 1000.0;
        let hour = (sec / 3600.0) as u64;
        let rem = sec % 3600.0;
        let min = (rem / 60.0) as u64;
        let sec = rem % 60.0;
        (hour, min, sec)
    }

    pub async fn run(&self) -> Result<()> {
        let ticket_id = self.account.ticket.id.clone();
        let perfomr_idx = self.account.ticket.sessions - 1; // 场次索引
        let sku_idx = self.account.ticket.grade - 1; // 票档索引

        info!("正在获取演唱会信息...");
        let ticket_info = self.get_ticket_info(ticket_id.clone()).await?;

        let ticket_name = ticket_info
            .detail_view_component_map
            .item
            .static_data
            .item_base
            .item_name;

        let perform_id = ticket_info
            .detail_view_component_map
            .item
            .item
            .perform_bases[perfomr_idx]
            .performs[0]
            .perform_id
            .clone();

        let perform_name = ticket_info
            .detail_view_component_map
            .item
            .item
            .perform_bases[perfomr_idx]
            .performs[0]
            .perform_name
            .clone();

        info!("正在获取场次/票档信息...");
        let perform_info = self.get_perform_info(ticket_id, perform_id).await?;
        let sku_id = perform_info.perform.sku_list[sku_idx].sku_id.clone();
        let sku_name = perform_info.perform.sku_list[sku_idx].price_name.clone();
        let item_id = perform_info.perform.sku_list[sku_idx].item_id.clone();

        let start_time_str = ticket_info
            .detail_view_component_map
            .item
            .item
            .sell_start_time_str;
        let mut  start_timestamp = ticket_info
            .detail_view_component_map
            .item
            .item
            .sell_start_timestamp
            .parse::<i64>()?;

        let request_time = self.account.request_time.unwrap_or(-1);

        let retry_times =  self.account.retry_times.unwrap_or(2);
        let retry_interval = self.account.retry_interval.unwrap_or(100);

        if request_time > 0 {
            start_timestamp = request_time
        }

        println!(
            "\r\n\t账号备注:{}\n\t门票名称:{}\n\t场次名称:{}\n\t票档名称:{}\n\t开抢时间:{}\n",
            self.account.remark, ticket_name, perform_name, sku_name, start_time_str
        );

        let (s, r) = async_channel::unbounded::<bool>();

        let interval = self.account.interval.unwrap_or(50);
        let earliest_submit_time = self.account.earliest_submit_time.unwrap_or(1);

        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    info!("CTRL-C, 退出程序...");
                    return Ok(());
                }

                _ = tokio::time::sleep(Duration::from_millis(interval)) => {
                    let local: DateTime<Local> = Local::now();
                    let millis = local.timestamp_millis();
                    let time_left_millis = start_timestamp - millis;
                    if time_left_millis <= earliest_submit_time {
                        let _ = s.send(true).await;
                    }else{
                        let (hours, minutes, seconds) = self.ms_to_hms(time_left_millis);
                        print!("\r\t开抢倒计时:{}小时:{}分钟:{:.3}秒\t", hours, minutes, seconds);
                        let _ =io::stdout().flush();
                    }

                    // info!("剩余时间毫秒:{}", time_left_millis);
                }

                _ = r.recv() => {

                    // 多次重试
                    for _ in 0..retry_times {
                        if let Ok(res) = self.buy(&item_id, &sku_id).await {
                            if res {// 抢购成功, 退出
                                return Ok(());
                            }
                        }
                        
                        // 重试间隔 
                        tokio::time::sleep(Duration::from_millis(retry_interval)).await;
                    }
                    return Ok(());
                }
            }
        }
    }
}
