# 环境
 
光猫------路由器merlin梅林 A--------路由器openwrt B
原来主要通过A拨号上网，B闲置。
为了探索透明代理，防止配置错误影响上网，计划在B上折腾，待条件成熟，考虑迁移到A上。
A的一个LAN口与B的一个LAN口相连，B从A获取IP，或者静态设置IP。 暂时关闭B的WAN拨号功能，并开启WIFI功能。这样可以连接WIFIA或者WIFEB。

B上若实现了透明代理功能(DNS功能和代理功能），这样客户端机器通过自动DHCP获取（连接WIFIB）或者手动设置route/gateway IP 为BIP，同时设置DNS为 BIP，就可以透明上网了。原有连接WIFIA的客户端还可以继续使用WIFIA的方式，保证了平滑过渡。



# 计划
## 基本原型
参考[透明代理TPROXY](参考v官方教程)。

## 优化
考虑性能、DNS分流方案、Dot、IPv6、Nftable

# 实践
## 基本原型
### B环境准备
B必须能上网才能做透明代理。
* BLAN1 与ALAN1相连
* 在/etc/config/network配置lan interface 为多IP模式。gateway ip是AIP。
```
list ipaddr '192.168.1.16/24'
list ipaddr '192.168.2.10/24'
option gateway '192.168.1.1'
```    
* 在/etc/config/network中 disable WAN,不要用DHCP，不然会从A中获取，污染B的路由设置。
* 配置dnsmasq，添加dns服务器
```
uci add_list dhcp.@dnsmasq[0].server="192.168.1.1"
uci commit dhcp
service dnsmasq restart
```  
当然可以web或者直接修改配置文件/etc/config/dhcp的方式来完成. 以上两操作保证在B上可以ping通 www.baidu.com.
* cat /proc/sys/net/ipv4/ip_forward    = 1
* 手动配置某个客户端，将默认网关指向BIP即 192.168.1.16。此时 应当能正常上网。
### v客户端安装和配置
* v版本选择下载，首先判断cpu型号和大端字节序或者小端字节序
```
root@LEDE:~# cat /proc/cpuinfo
system type		: Broadcom BCM4716
machine			: Asus RT-N16
processor		: 0
cpu model		: MIPS 74Kc V4.0
BogoMIPS		: 239.61
wait instruction	: yes
microsecond timers	: yes
tlb_entries		: 64
extra interrupt vector	: yes
hardware watchpoint	: yes, count: 4, address/irw mask: [0x0ffc, 0x0ffc, 0x0ffb, 0x0ffb]
isa			: mips1 mips2 mips32r1 mips32r2
ASEs implemented	: mips16 dsp dsp2
Options implemented	: tlb 4kex 4k_cache prefetch mcheck ejtag llsc dc_aliases vint perf_cntr_intr_bit nan_legacy nan_2008 perf
shadow register sets	: 1
kscratch registers	: 0
package			: 0
core			: 0
VCED exceptions		: not available
VCEI exceptions		: not available
root@LEDE:~# hexdump -s 5 -n 1 -C /bin/busybox
00000005  01                                                |.|
00000006
root@LEDE:~# echo -n I | hexdump -o | awk '{ print substr($2,6,1); exit}'
1 //1 mean 小端字节序
```
* v 安装
  v解压后60MB，但看B的空间，通过删除v解压后的重复文件可以放到/tmp下,有采用自行编译并用upx来压缩的方式，为了快速达到目标，这次用了usb盘
```
root@LEDE:~# df -h
Filesystem                Size      Used Available Use% Mounted on
/dev/root                 2.5M      2.5M         0 100% /rom
tmpfs                    61.1M      1.1M     60.0M   2% /tmp
/dev/mtdblock5           27.8M    996.0K     26.8M   4% /overlay
overlayfs:/overlay       27.8M    996.0K     26.8M   4% /
tmpfs                   512.0K         0    512.0K   0% /dev

opkg update && opkg install block-mount e2fsprogs kmod-fs-ext4 kmod-usb-storage kmod-usb2 kmod-usb3
block detect | uci import fstab
uci set fstab.@mount[0].enabled='1' && uci set fstab.@global[0].anon_mount='1' && uci commit
/sbin/block mount && service fstab enable
#copy to B 
scp ~/Downloads/v-linux-mipsle.zip root@192.168.1.16:/mnt/sdav/v
```
* run v
```
  root@LEDE:~# opkg install curl
  # config from system proxy or 参考透明代理tproxy的v客户端配置 
  root@LEDE:/mnt/sda/v# ./v -c config.jsonsystemproxy  
  root@LEDE:~# curl -so /dev/null -w "%{http_code}" xxxx.com -x socks5://127.0.0.1:1080
  # 确保301/200就行。 
``` 
* config v for tproxy 
``` 
  root@LEDE:/mnt/sda/v# opkg install iptables-mod-tproxy ipset
  # maybe we will use ipset later
 
  root@OpenWrt:/mnt/sda/v# cat mytest3.sh
  #!/bin/sh
  
  ip rule add fwmark 1 table 100 # add ip rule ,once a packet match the fwmark = 1,then lookup the route table 100.
  ip route add local 0.0.0.0/0 dev lo src 127.0.0.1 table 100 # add local type's route entry to route table  100
  
  iptables -t mangle -N ONLYONE
  iptables -t mangle -A ONLYONE -j MARK --set-mark 1
  iptables -t mangle -A ONLYONE -j ACCEPT
  iptables -t mangle -A PREROUTING -m socket -j ONLYONE // prevent packet which have existing connectin to enter into tproxy twice
  
  iptables -t mangle -N V
  iptables -t mangle -A V -d 127.0.0.1/32 -j RETURN
  iptables -t mangle -A V -d 224.0.0.0/4 -j RETURN
  iptables -t mangle -A V -d 255.255.255.255/32 -j RETURN
  iptables -t mangle -A V -d 192.168.0.0/16 -p tcp -j RETURN
  iptables -t mangle -A V -d 192.168.0.0/16 -p udp ! --dport 53 -j RETURN # let port 53 udp packet enter into the v app
  iptables -t mangle -A V -p udp -j TPROXY --on-port 13345 --on-ip 127.0.0.1 --tproxy-mark 0x1/0x1 # use tproxy to forward the packet to local port 13345,meantime mark 1
  iptables -t mangle -A V -p tcp -j TPROXY --on-port 13345 --on-ip 127.0.0.1 --tproxy-mark 0x1/0x1
  iptables -t mangle -A PREROUTING -j V // let other new  packet  processing by the v chain
  
  iptables -t mangle -N V_MASK
  iptables -t mangle -A V_MASK -d 224.0.0.0/4 -j RETURN
  iptables -t mangle -A V_MASK -d 255.255.255.255/32 -j RETURN
  iptables -t mangle -A V_MASK -d 192.168.0.0/16 -p tcp -j RETURN
  iptables -t mangle -A V_MASK -d 192.168.0.0/16 -p udp ! --dport 53 -j RETURN
  iptables -t mangle -A V_MASK -j RETURN -m mark --mark 0xff # let packet which have mark 255 to direct connection
  iptables -t mangle -A V_MASK -p udp -j MARK --set-mark 1
  iptables -t mangle -A V_MASK -p tcp -j MARK --set-mark 1
  iptables -t mangle -A OUTPUT -j V_MASK
```
  用执行脚本的方式来验证。待验证成功后，可以放到/etc/firewall.user持久化。v 客户端的配置暂时就是参考链接里的配置。
* 参考网上配置把v做成服务的方式，系统启动后自动启动。

### 注意事项
* B openwrt failsafe mode
  一开始最好调研好怎么进入这个failsafe模式。同时执行时尽量避免把路由器变砖了。
  本人使用iptable，把规则放到了`/etc/firewall.user`中，然后再也没法访问了，最后只能进入华硕的恢复模式重刷openwrt固件。折腾了半天也没进入openwrt的failsafe mode。

## 优化
### DNS over tls
DoH虽然因为浏览器的普及很容易实现和普及，但据说有泄露agent的可能。综合多种原因，最终选择了Dot。
#### 在A路由器merlin上配置Dot
参考[DNS privacy ](https://github.com/RMerl/asuswrt-merlin.ng/wiki/DNS-Privacy)
在android和pc 上设置dns ip 为AIP 192.168.1.1.
打开网页测试https://www.cloudflare.com/ssl/encrypted-sni/
这里的设置分为路由（网关）和DNS单独的设置。
所以虽然B透明代理当前也劫持53的DNS流量，但是我们仍然可以用A作为DNS服务器，只是需要手动设置而已。
#### Dot+ESNI
ESNI属于web服务器的客户端范畴的概念，是TLSv1.3的一个扩展功能,加密我们要访问的服务器名字。
1. 支持情况：
   * firefox：已经73版本后支持。
   * chrome：不支持，参见[Support for Encrypted SNI (ESNI)](https://bugs.chromium.org/p/chromium/issues/detail?id=908132) 
2. 测试验证：
   * firefox
     整体结果不理想。
     Mac采用Dot DNS,即设置DNS IP为路由器AIP 192.168.1.1（路由器DNS禁用了doh）。在about：config里设置network.trr.mode 0/2/3/5多种情况下，偶尔发现启用了ESNI。但大多数情况下都失败着。
     采用wireshark抓包、[cloudflare check](https://www.cloudflare.com/ssl/encrypted-sni/)、 [cloudflare trace api](https://www.cloudflare.com/cdn-cgi/trace)多种验证方法。
     结果不乐观的原因：1， 启用ESNI必须启用Doh，即Doh是依赖。 参见[bug](https://bugzilla.mozilla.org/show_bug.cgi?id=1500289). 2,  其他干扰因素比如权威DNS服务器与TLS服务器不同步的话会出现这个问题。
     总之并不能愉快的启用ESNI，特别是在开启Dot的情况下。暂时放弃。

#### Dot over V

### 性能
### DNS分流方案
### IPv6
### IPv6 Nftable

# 一些概念和疑惑
* router
Any machine which will accept and forward packets between two networks is a router.Every router is at least dual-homed; one interface connects to one network, and a second interface connects to another network.
* route -n == netstat -rn
* ip rule
``` 
root@OpenWrt:/mnt/sda/v# ip rule
0:	from all lookup local
32765:	from all fwmark 0x1 lookup 100
32766:	from all lookup main
32767:	from all lookup default
```
  ip rule可以查看有那些路由规则,32765表示优先级， 系统一般默认有路由表main，local，和default。 这里lookup 前的是规则选择的方式，lookup是动作，表示一旦一个packet满足规则，就会lookup某个路由表。 路由表可以有名字或者叫table 100这样的。

* The route selection algorithm under linux
  分为基于目的地址的路由和基于路由策略的路由。
  高级路由也叫策略路由。
  首先在路由策略数据库rpdb中根据优先级进行迭代遍历，利用longest prefix match selection algorithm 试图找到对应的路由，若找到，就会转发那个包，否则进入下一个规则。
  main路由表里一般包含默认路由，即所有其他IP走这个路由。
* When do reroute check?
  grep "Reroute for ANY change" net/ipv4/netfilter/iptable_mangle.c.
  Basically "any" means saddr,daddr mark and tos, the four routing-influencing parameters.
  在上面配置output链规则的时候，一旦有这几项改变，会导致包的重路由，不再是原来的正常的路径。
  当然reroute check的执行不一定只在output链这里。
* 为什么在路由策略里那里添加src 127.0.0.1
  ip route add local 0.0.0.0/0 dev lo src 127.0.0.1 table 100 # 
  Make it explicit that the source IP used for this network when connecting locally should be in 127.0.0.0/8 range. This is needed since otherwise the TPROXY rule would match both forward and backward traffic. We want it to catch forward traffic only.
  ref [cloudflare blog](https://blog.cloudflare.com/how-we-built-spectrum/)
  还没明白，但是正常使用来着现在。
* 为什么要添加match那条规则
  [tproxy](https://www.kernel.org/doc/Documentation/networking/tproxy.txt) 中有说明。
  主要是防止已经有连接的包不管TCP还是UDP，不必要再次进入tproxy，只是标记为1，让策略路由把此包根据mark=1的规则来路由。
  估计有性能提升，有兴趣的可以测试下。
* How does dnsmasq interaction with dot?
* openwrt fw3 and nftable cheatsheet
* why fail use this?
  期间遇到的问题，还好解决了，但现在不清楚为啥？
  为了引入match那个规则，采用如下方式
  步骤二执行后，dns不可用，但外网ip可以ping 通。执行步骤三但最后一个规则忽略。然后执行那个规则，会导致服务器无法连接。
  是不是iptable rule执行顺序的问题
``` 
# 操作要点： 
主要添加了ONLYONE链，作用是防止同属于一个连接的其后的包进入tproxy两次。
还有在步骤三中,添加了iptables -t mangle -A BYPAS_MASK -d 127.0.0.1/32 -j RETURN
原来参考中，是没有这一句的，原来步骤二三中的直连，不同的就是这一句。
v client 配置用就是那个官方tproxy教程的配置，inbounds没有开放53端口，操作系统有自己的dns应用。 

# 步骤
## 一 策略路由
ip rule add fwmark 1 table 100
ip route add local 0.0.0.0/0 dev lo table 100

## 二 外面发过来的的包到透明网关

iptables -t mangle -N BYPASS
iptables -t mangle -A BYPASS -d 127.0.0.1/32 -j RETURN
iptables -t mangle -A BYPASS -d 224.0.0.0/4 -j RETURN
iptables -t mangle -A BYPASS -d 255.255.255.255/32 -j RETURN

iptables -t mangle -A BYPASS -d 192.168.0.0/16 -p tcp -j RETURN
iptables -t mangle -A BYPASS -d 192.168.0.0/16 -p udp ! --dport 53 -j RETURN


iptables -t mangle -N ONLYONE
iptables -t mangle -A ONLYONE -j MARK --set-mark 1
iptables -t mangle -A ONLYONE -j ACCEPT

iptables -t mangle -N V
iptables -t mangle -A V -p udp -j TPROXY --on-port 12345 --tproxy-mark 1
//用tproxy转发到127.0.0.1 13345，即让v来处理
iptables -t mangle -A V -p tcp -j TPROXY --on-port 13345 --tproxy-mark 1


iptables -t mangle -A PREROUTING -j BYPASS  //BYPASS掉一些私有/内网地址

**iptables -t mangle -A PREROUTING -m socket -j ONLYONE**
// 添加ONLYONE链到mangle表的prerouting链里，即应用ONLY链的规则。 防止同属于一个连接的其后的包进入tproxy两次，
// 这里让它直接进入ONLYONE链里处理，即直接路由到loop，不再经过tproxy，即不会执行下面这个rule了

iptables -t mangle -A PREROUTING -j V 
// 添加V链到mangle表的prerouting链里，即应用规则，即给 UDP/TCP 打标记 1，用tproxy转发至v app 的 13345 端口并开始路由决策


## 三 透明网关自身生成的包： v生成的包要发出去，或者还有自身生成的其他包

iptables -t mangle -N BYPASS_MASK
**iptables -t mangle -A BYPAS_MASK -d 127.0.0.1/32 -j RETURN**
iptables -t mangle -A BYPASS_MASK -d 224.0.0.0/4 -j RETURN
iptables -t mangle -A BYPASS_MASK -d 255.255.255.255/32 -j RETURN

iptables -t mangle -A BYPASS_MASK -d 192.168.0.0/16 -p tcp -j RETURN
iptables -t mangle -A BYPASS_MASK -d 192.168.0.0/16 -p udp ! --dport 53 -j RETURN

iptables -t mangle -N V_MASK
iptables -t mangle -A V_MASK -j RETURN -m mark --mark 0xff //v生成的带mark的包让其直接发出
iptables -t mangle -A V_MASK -p udp -j MARK --set-mark 1 // 这可能是哪个app的流量？ 要重新路由到loop
iptables -t mangle -A V_MASK -p tcp -j MARK --set-mark 1 // 这是系统dns的s的流量？要重新路由到loop

iptables -t mangle -A OUTPUT -j BYPASS_MASK //应用bypass规则
**iptables -t mangle -A OUTPUT -j V_MASK** 
```





