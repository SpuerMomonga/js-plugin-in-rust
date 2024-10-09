// 循环接收主进程发送的消息
process.on("message", (m) => {
  process.send(`Hello, ${m}! You've been greeted from node!`);
});
