const streams = require('./lib')

console.log("streams", streams)
console.log("get_node_info", streams.get_node_info())

let hello = streams.hello()

console.log("Greetings: ", hello)

streams.publish(hello).then(success => {
     console.log("Greetings published successfully ", success)
}).catch(error => {
     console.log("Greetings published error ", error)
})


