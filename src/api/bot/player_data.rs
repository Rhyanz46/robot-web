use std::collections::HashMap;
use actix_web::cookie::SameSite;
use thirtyfour::prelude::*;

// struct MidasbuySession {
//     cookie_control: &'static str
// }

#[derive(Clone)]
pub struct PlayerData<'a> {
    pub pubg_id: &'a str,
    pub hp_selected: &'a str,
    pub uc_selected: &'a str,
}

static EMAIL: &str = "bravemarioline@gmail.com";
static PASSWORD: &str = "brave.12345";


impl <'a>PlayerData<'a> {
    pub async fn buy(&'a self){
        
        let mut paket_index = 0;
        let mut selected = false;
        let paket_list: [&str;9] = ["15+1 UC","25+1 UC","50+2 UC","100+5 UC","125+6 UC","250+13 UC","500+30 UC","750+75 UC","1000+100 UC"];

        for (index, item) in paket_list.as_slice().iter().enumerate(){
            if self.uc_selected == *item{
                paket_index = index;
                selected = true;
                break;
            }
        }
        if !selected{
            println!("paket salah");
            return
        }
        let mut caps = DesiredCapabilities::chrome();
        match caps.add_chrome_arg("--disable-features=IsolateOrigins,site-per-process"){
            Ok(_) => (),
            Err(_) => println!("error running on mode non isolate")
        };
        match caps.add_chrome_arg("--headless"){
            Ok(_) => (),
            Err(_) => println!("error running background")
        };
        let driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();
        // let cookies = login_fb(driver.clone()).await;
        driver.goto("https://www.midasbuy.com/midasbuy/id/buy/pubgm?sc=os_upoint&from=__mds_buy_duniagames").await.unwrap();
        
        // for item in cookies.as_slice().iter(){
        //     println!("{}, {}", item.name(), item.value());
        //     let tem_cookie = Cookie::new(item.name(), item.value());
        //     driver.add_cookie(item.clone()).await.unwrap();
        // }
    
    
        let a = driver.find_all(By::ClassName("close-btn")).await.unwrap();
        let aa = a.last();
        match aa {
            Some(aval) => {
                aval.click().await.unwrap();
            },
            None=> {
                driver.clone().quit().await.unwrap();
                println!("tidak ada close button");
                return
            }
        }
    
        let b = driver.find(By::ClassName("eea-pop")).await.unwrap();
        let bb = b.find(By::ClassName("close-btn")).await.unwrap();
        bb.click().await.unwrap();
    
        self.login_basic(driver.clone()).await;
        println!("login success");
    
        // select_login_fb(driver.clone()).await;
    
    
        // let timeouts = driver.get_timeouts().await.unwrap();
        // println!("Sign Page load timeout = {:?}", timeouts.page_load());
        // driver.refresh().await.unwrap();
    
        // driver.add_cookie(cookie_datr).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;

        // let mut cookie_datr = Cookie::new("arian", "test");
        // cookie_datr.set_domain("midasbuy.com");
        // cookie_datr.set_path("/");
        // cookie_datr.set_same_site(Some(SameSite::Lax));

        self.input_id_and_select_item(driver.clone(), true, paket_index).await.unwrap();
    
        // proses buy
        let handles = driver.windows().await.unwrap();
        driver.switch_to_window(handles[1].clone()).await.unwrap();
        driver.enter_frame(1).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await; // we must wait, because frame is loading
        let el = driver.active_element().await.unwrap();
        let mut ell = el.find(By::ClassName("content---channelItem---14lSv")).await.unwrap();
        ell.click().await.unwrap();
        ell = el.find(By::ClassName("content---blockContent---1BSJB")).await.unwrap();
        println!("memasukkan nomor : {:?}", self.hp_selected);
        ell.send_keys(self.hp_selected).await.unwrap();

        driver.execute("document.getElementsByClassName('content---footerBtn---1AXD0')[1].click()", Vec::new()).await.unwrap();
        println!("berhasil di beli");
    
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    
        driver.quit().await.unwrap();
    }

    async fn input_id_and_select_item(&'a self, driver: WebDriver, buy: bool, paket: usize) -> Result<String, String>{
        let player_name;
        let form_input = driver.find_all(By::ClassName("input")).await.unwrap();
        let paket_list = driver.find(By::ClassName("game-pay-section")).await.unwrap();
        let val = form_input.first().unwrap();
        val.send_keys(self.pubg_id).await.unwrap();
        let paket_list_opt = paket_list.find_all(By::Tag("li")).await.unwrap();
        println!("{:?}", paket_list_opt[paket].outer_html().await);
        if paket_list_opt[paket].is_clickable().await.unwrap(){
            paket_list_opt[paket].click().await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            let btnbuy = driver.find(By::Id("buy-payBtn")).await.unwrap();
            if btnbuy.is_clickable().await.unwrap(){
                let box_player_res = driver.find(By::ClassName("user-head")).await;
                match box_player_res {
                    Ok(box_player) => {
                        let player_name_box = box_player.find(By::ClassName("name")).await.unwrap();
                        player_name = player_name_box.inner_html().await.unwrap();
                        if !buy{
                            return Ok(player_name);
                        }
                    },
                    Err(_) => {
                        driver.quit().await.unwrap();
                        return Err("ID tidak valid".to_string());
                    }
                }
                btnbuy.click().await.unwrap();
            }else{
                driver.quit().await.unwrap();
                return Err("Tidak dapat menekan tombol buy".to_string());
            }
        }else{
            driver.quit().await.unwrap();
            return Err("tidak bisa memilih paket".to_string());
        }
        Ok(player_name)
    }

    #[allow(dead_code)]
    async fn login_fb(&self, driver: WebDriver) -> Vec<Cookie<'a>>{
        driver.goto("https://www.facebook.com").await.unwrap();
    
        let email_input = driver.find(By::Css("input[name=email]")).await.unwrap();
        email_input.send_keys(EMAIL).await.unwrap();
        let password_input = driver.find(By::Css("input[name=pass]")).await.unwrap();
        password_input.send_keys(PASSWORD).await.unwrap();
    
        let login_button = driver.find(By::Css("button[name=login]")).await.unwrap();
        login_button.click().await.unwrap();
        let timeouts = driver.get_timeouts().await.unwrap();
        println!("Login Page load timeout = {:?}", timeouts.page_load());
    
        // Wait for the page to load after logging in.
        // driver.wait_for_navigation(None, None).await.unwrap();
    
        // Retrieve the cookies from the browser window.
        let cookies = driver.get_all_cookies().await.unwrap();
    
        // Return the cookies retrieved from the browser window.
        return cookies
        // driver.goto("https://wikipedia.org").await.unwrap();
        // let mut cookie = Cookie::new("key", "value");
        // cookie.set_domain("wikipedia.org");
        // cookie.set_path("/");
        // cookie.set_same_site(Some(SameSite::Lax));
        // driver.add_cookie(cookie.clone()).await.unwrap();
        // println!("{:?}", cookie.name_raw());
    }
    
    async fn login_basic(&self, driver: WebDriver){
        driver.execute("
        document.getElementById('loginedUser').click();
        document.getElementsByClassName('headerLoginButton')[0].click();
        ", Vec::new()).await.unwrap();
    
        driver.enter_frame(2).await.unwrap();
        let data = driver.find_all(By::ClassName("input-box")).await;
        match data {
            Ok(d) => {
                let email_om = d[0].query(By::Tag("input[type=text]")).first().await.unwrap();
                let password_dom = d[1].query(By::Tag("input[type=password]")).first().await.unwrap();
                email_om.send_keys(EMAIL).await.unwrap();
                password_dom.send_keys(PASSWORD).await.unwrap();
                let el = driver.find(By::ClassName("sign-in-btn")).await.unwrap();
                el.click().await.unwrap();
                
                // let cookies_res = driver.get_all_cookies().await;
                // match cookies_res {
                //     Ok(cookie) => {
                //         for (i, c) in cookie.as_slice().iter().enumerate(){
                //             println!("{}. {} = {} | {}", i, c.name(), c.value(), c.domain().unwrap());
                //         }
                //         // driver.add_cookie(cookie[0].clone()).await.unwrap();
                //     },
                //     Err(_) => {
                //         println!("errror to get cookie");
                //         // driver.quit().await.unwrap();
                //     }
                // }
            },
            Err(_) => {
                println!("error");
                driver.quit().await.unwrap();
            }
        }    
    }
    
    #[allow(dead_code)]
    async fn select_login_fb(&self, driver: WebDriver){
        driver.execute("
        document.getElementById('loginedUser').click();
        document.getElementsByClassName('headerLoginButton')[0].click();
        ", Vec::new()).await.unwrap();
        driver.execute(
            "document.getElementById('login-iframe').contentDocument.getElementsByClassName('facebook-log-btn')[0].click()",
         Vec::new()
        ).await.unwrap();
    }

    pub async fn check_id(&'a self) -> Result<String, String>{
        let mut caps = DesiredCapabilities::chrome();
        match caps.add_chrome_arg("--disable-features=IsolateOrigins,site-per-process"){
            Ok(_) => (),
            Err(_) => println!("error running on mode non isolate")
        };
        match caps.add_chrome_arg("--headless"){
            Ok(_) => (),
            Err(_) => println!("error running background")
        };
        let driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();
        driver.goto("https://www.midasbuy.com/midasbuy/id/buy/pubgm?sc=os_upoint&from=__mds_buy_duniagames").await.unwrap();
        let a = driver.find_all(By::ClassName("close-btn")).await.unwrap();
        let aa = a.last();
        match aa {
            Some(aval) => {
                aval.click().await.unwrap();
            },
            None=> println!("tidak ada close button")
        }
        let b = driver.find(By::ClassName("eea-pop")).await.unwrap();
        let bb = b.find(By::ClassName("close-btn")).await.unwrap();
        bb.click().await.unwrap();
        self.login_basic(driver.clone()).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        return self.input_id_and_select_item(driver.clone(), false, 0).await;
    }
}