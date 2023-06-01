# dm-ticket
## 简介

大麦网自动购票, 支持docker一键部署。

**这软件没用的，不要Star了，不要fork了，不要再尝试了。**


## 特别声明
- 本项目内所有资源文件，禁止任何公众号、自媒体进行任何形式的转载、发布。
- 编写本项目主要目的为学习和研究Rust，无法保证项目内容的合法性、准确性、完整性和有效性。
- 本项目涉及的数据由使用的个人或组织自行填写，本项目不对数据内容负责，包括但不限于数据的真实性、准确性、合法性。使用本项目所造成的一切后果，与本项目的所有贡献者无关，由使用的个人或组织完全承担。
- 本项目中涉及的第三方硬件、软件等，与本项目没有任何直接或间接的关系。本项目仅对部署和使用过程进行客观描述，不代表支持使用任何第三方硬件、软件。使用任何第三方硬件、软件，所造成的一切后果由使用的个人或组织承担，与本项目无关。
- 本项目中所有内容只供学习和研究使用，不得将本项目中任何内容用于违反国家/地区/组织等的法律法规或相关规定的其他用途。
- 所有基于本项目源代码，进行的任何修改，为其他个人或组织的自发行为，与本项目没有任何直接或间接的关系，所造成的一切后果亦与本项目无关。
- 所有直接或间接使用本项目的个人和组织，应24小时内完成学习和研究，并及时删除本项目中的所有内容。如对本项目的功能有需求，应自行开发相关功能。
- 本项目保留随时对免责声明进行补充或更改的权利，直接或间接使用本项目内容的个人或组织，视为接受本项目的特别声明。

## 使用说明

- 下载docker-compose配置文件: `wget https://github.com/ClassmateLin/dm-ticket/releases/download/v0.1.3/dm-ticket.zip`
- 解压zip: `unzip dm-ticket.zip && cd dm-ticket`
- 运行容器: `docker-compose up -d`
- 修改配置: `vim config/config.yaml`, 配置项在config/config.yaml中有详细注释。
- 运行脚本: `docker exec -it dm-ticket dm-ticket`
  - sample 1:
     ![run.png](./images/run.png)
     ![run_res.png](./images/run_res.jpeg)
  - sample 2:
    ![run2.png](./images/run2.png)


## 命令列表

- 自动购票: `docker exec -it dm-ticket dm-ticket`
- 扫码登录: `docker exec -it dm-ticket dm-login`


    
## 常见问题

- 如遇到`Connection refused (os error 111)`错误, 说明token-server还没启动完成, 等待片刻即可。
![Connection refused (os error 111)](./images/connection_errors.png)
- 生成订单失败, ["RGV587_ERROR::SM::哎哟喂,被挤爆啦,请稍后重试!"], 请检查是否复制了完整的cookie, ip有问题(一般是使用了大厂服务器/代理, 实在需要可以使用这种[动态VPS](https://www.fwvps.com/?aff=6bb13))。
- ["B-00203-200-100::网络开小差了，再试一次吧~"], 请检查是否复制了完整的cookie。
- docker/docker-compose安装使用问题，请善用搜索引擎, 自行搜索解决方案。
- 是否支持多账号, v0.1.0版本是支持多账号的。后续可能取消。要实现多账号支持, 开启多个docker容器也可以支持。
- 频繁尝试运行程序出现,  ["RGV587_ERROR::SM::哎哟喂,被挤爆啦,请稍后重试!"]。请重新登陆。
- 仅支持[H5端](https://m.damai.cn)可以购买的票。


## 其他说明

- 如何获取cookie? 

  1. 登录[大麦网](https://m.damai.cn/), F12打开控制台查看网络请求, 复制请求中的cookie。 
  ![img.png](images/cookie.png)
  2. 使用扫码登录: `docker exec -it dm-ticket dm-login`

- 如何获取演唱会id？
 
 进入门票详情, 复制URL中的itemId。
 ![ticket_id](./images/ticket.png)

- 如何获取场次？

 点击购买按钮, 弹出的场次。第一个就是1, 以此类推。
 ![img.png](images/session_id.png)

- 如何获取票档?

 选择场次之后, 弹出票档信息, 从左到右, 从上到下, 从1开始递增。如图:
![img.png](images/grade.png)

- 实名信息怎么选择?

 按实名信息顺序, 自动选择。 如购买2张票, 默认选择前两位实名人。


