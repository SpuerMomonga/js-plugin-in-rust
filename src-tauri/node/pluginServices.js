import process from 'node:process';

process.stdin.on("data", (data) => {
  const message = data.toString().trim();
  // 模拟处理消息后，发送回复给父进程
  process.stdout.write(`Reply from Node.js: Processed message '${message}'\n`);
});

setInterval(() => {
  // process.stdin.on("data", (data) => {
  //   const message = data.toString().trim();
  //   // 模拟处理消息后，发送回复给父进程
  //   process.stdout.write(
  //     `Reply from Node.js: Processed message '${message}'\n`
  //   );
  // });
  // console.log("hello from node");
}, 5000);
