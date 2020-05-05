const {hello, publish} = require('./../native');

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
    publish: _publish
}