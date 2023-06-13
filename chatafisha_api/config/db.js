// call mysql
const mysql = require('mysql2');

// db properties
const props = require('./props');
const info = props.DB;
var db = mysql.createPool({
    host:info.HOST,
    user:info.USER,
    password:'',
    database:info.DBNAME
})

// check connection
db.getConnection((err)=>{
    if(!err){
        console.log("aset: DB connected!");
    }else{
        console.log(`aset: DB failed to connect due to: ${err}`);
    }
});

module.exports=db;


