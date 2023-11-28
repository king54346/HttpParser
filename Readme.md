使用pnet调用底层的packet socket 实现http流量的抓取，由于抓取的是tcp的原始流量， 没有经过tcp协议栈(滑动窗口等技术)，导致需要自己处理组包，重传，乱序等。
packet socket 的fanout功能可以将流量分发到多个线程中，并且有多种的方式，例如轮询(将packet分别分发给每个线程)，hash(根据每个packet的hash，将相同的hash分发到同一个线程当中)等。
