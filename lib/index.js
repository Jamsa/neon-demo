var addon = require('../native');

console.log(addon.hello());
console.log(addon.add(1,2,3,4,5,20,12.2344566));
console.log(JSON.stringify(addon.objTest()));
console.log('cbTest调用结果',addon.cbTest(function(a,b){
    console.log('cb arg:',a,b);
    return a+100;
}))

