# iota-streams-js

Native Node.js modules with IOTA Streams Rust binding


Install via npm
```bash
npm install iota-streams-js
```

```bash
const streams = require('iota-streams-js')

streams.publish("Hello").then(success => {
     console.log("Greetings published successfully ", success)
}).catch(error => {
     console.log("Greetings published error ", error)
})
```

Run the demo
```bash
git clone https://github.com/huhn511/iota-streams-js
cd iota-streams-js
npm install
node demo.js 
```