mod common;
mod io;
mod client;

fn main() {

    println!(">>> sending mail");

    let mut email = client::Email::new();
    email.name(String::from("gzbakku"));
    email.from(String::from("akku@silvergram.in"));
    email.to(String::from("gzbakku@localhost:25"));
    email.subject(String::from("hello world"));
    email.body(String::from("first message\nsecond message\nthird message"));
    // email.body(String::from("<html> <header><title>This is title</title></header> <body> Hello world </body> </html>"));
    // email.attach("d://workstation/expo/rust/letterman/run.bat".to_string());
    // email.attach("d://workstation/expo/rust/letterman/sample.txt".to_string());
    // email.attach("d://workstation/expo/rust/letterman/MailHog_windows_386.exe".to_string());
    // email.is_html();

    // email.attach("C:\\Users/tejas/Downloads/drink.png".to_string());

    // let base64_data = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAAdgAAAHYBTnsmCAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAJfSURBVDiNpZLdS1MBGMafc87OPtx23M7Opse2hDb8mB+ZLRC70IhBd4GSJAUVYnXVXyBBEHST1xZRmUgXgdHFiC6KwixCyjLDmTKx4cm17Ux2trntnO3YTYrOFYTP3fvwvj8eeF5gn6L+4pPtza6bvJ3xrcWkd/9NrXXaep49vCKP3Dqbomn68Cl/w52BC8emJsYvdpTuasoB0tl88Gngc0TK5GRFUSrP9fou9fZ5tQ8eTY8COAIg+0+AKKaDYxPTbQAUAAeK+U0SKoGVlXXDH29bxM6Bc/B3K4zWWlUtZIuKnC0UCxsESSotddZBzyEXNbcQTi2GxMfxqHC1XAJSpzd1eVs76g0VZtC0dleqaE4G43CYdcLbbgAkALUUoArhpf4ui+Wj286TyXwOObUIANCTFCp1OoRiEVUIL/VvHW8DWI4fZBh2QJLEF7yd/9njO+7UECTYCiMAILGRQWFTxfinKYHlqk8zjG1EkhL3E/G1exqLveqat7njRo3LY1lbDfli+bQa2chg6ZeA+UQcAIEmloOnqgbRXL66qbVziHe6qdXwYv383Ac9yZisgzUujwUATJUspXewtH94CG5vI2aTEmaTSbibGuEfHoLBztJmhqUAwHmwzsIw7GXN+np89OXzsQaG4VoUOZdWOFtbKjBpO2ricLvLDwBwG21IBSaxHFoWgwnxC63VmyQpPqcUisFdNQLAyfrm133tnd3l/uPJzPs3r75/O7HT2/NIZkpbrCUIGEtqTMt5GAhSLd3fA5gRQmd+1LVcD0uCOyRGrVt+RpGlwMLX8+WS7Uu/AV/Q4yOF5rS7AAAAAElFTkSuQmCC".to_string();
    // email.attach_base64("drink.png".to_string(),base64_data);

    match email.send(){
        Ok(_)=>{
            println!("email sent");
        },
        Err(e)=>{
            println!("email failed : {:?}",e);
        }
    }

}
