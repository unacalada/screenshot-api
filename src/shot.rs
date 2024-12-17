use std::fs;
use headless_chrome::{Browser, protocol::page::ScreenshotFormat};



use std::error::Error;



pub struct Options {
    pub url: String,
    pub output_file: String,

}

pub fn capture(option: Options) -> Result<(), Box<dyn Error>> {
  //  let rt = Runtime::new()?;

  
       
        let browser = Browser::default()?;

        let tab = browser.wait_for_initial_tab()?;

       
        tab.navigate_to(&option.url)?;

     
        tab.wait_until_navigated()?;

       
        let png_data = tab.capture_screenshot(
            ScreenshotFormat::PNG,
            None,
            true, 
        )?;

     
        fs::write(&option.output_file, png_data)?;

        Ok(())
    
}
