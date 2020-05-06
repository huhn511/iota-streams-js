const {hello, publish, get_node_info} = require('./../native');

// Iterate 10,0000 times in background thread
_publish = () => {
    return new Promise((resolve, reject) => {
        publish((err, res) => {
        if(err) reject(err)
        resolve(res)
        });
    });
};

//console.log(addon.hello());
module.exports = {
    hello,
    get_node_info,
    publish: _publish
}