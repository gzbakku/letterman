const nodemailer = require("nodemailer");
const fs = require('fs');

main();

async function main(){

  const private_key = read_file("../keys/private.key");

  let transporter,to;
  if(true){
    to = "9mvxra2pyfknnz@dkimvalidator.com";
    transporter = nodemailer.createTransport({
      host: "31045262.in1.mandrillapp.com",
      port: 25,
      secure: false,
      dkim: {
        domainName: "silvergram.in",
        keySelector: "dkim",
        privateKey: private_key
      }
    });
  } else {
    to = "akku@localhost";
    transporter = nodemailer.createTransport({
      host: "localhost",
      port: 25,
      secure: false,
      dkim: {
        domainName: "silvergram.in",
        keySelector: "dkim",
        privateKey: private_key
      }
    });
  }

  let info = await transporter.sendMail({
    from: 'gzbakku@silvergram.in', // sender address
    to: to, // list of receivers
    subject: "Hello world", // Subject line
    text: "first message\r\nsecond message\r\nthird message\r\n", // plain text body
    html: "<b>Hello world?</b>", // html body
  });

  console.log(info);

}

function read_file(path){
  return fs.readFileSync(path, 'utf8');
}

function get_private_key(){

}
