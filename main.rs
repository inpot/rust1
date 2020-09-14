use std::fs::File;
use std::fs;
use std::io::Read;
use std::io::Error;
use std::io;
use std::env;
use std::io::BufReader;
use std::{thread, io::prelude::*, time::{SystemTime, Duration}, ops::Deref, fmt::{self, Display}, rc::Rc, sync::{Mutex, mpsc, Arc}};
use mpsc::{Sender, channel};
use std::future::Future;
use fmt::Debug;
use std::net;
use std::{ os};
use tokio::task;
use reqwest::Proxy;

mod thread_pool;
fn gfwlist2Dnsmasq() {
  //let andr = os::android;

  let dns= "127.0.0.1#5353";
  let ipset= "gfwlist";
  //let res = reqwest::get("http://pl.goinbowl.com").await.ok().unwrap().text().await.ok().unwrap();
     //let res = reqwest::blocking::get("http://127.0.0.1:8088").unwrap().text().unwrap();
  let content = get_gfwlist();
  match content {
      Ok(txt) => {
        println!("Text  {}",txt);
        let mut resultStr = String::new(); 
        let mut line_size = 0;
        let mut  ignored =String::new();
        for line in txt.lines(){ 
          if line.starts_with("." ){
            let target = line[1..line.len()].to_string(); 
            processLine(&mut resultStr, &target, dns, ipset)
          }else if line.starts_with("||"){ 
            let target = line[2..line.len()].to_string(); 
            processLine(&mut resultStr, &target, dns, ipset) 
          }else{
            println!("Do not proccessed line: {}",line);
            ignored.push_str(line);
            ignored.push_str("\n");
            continue;
          } 
        } 
        let filePath = "gfwlist.conf";
        let ignoredPath= "ignored.conf";
        
        let res1 = write_file(ignored, &ignoredPath);
        //let mut file = File::open(filePath).unwrap()//.unwrap_or_else(||{File::create(filePath)});
        let res = write_file(resultStr, &filePath);
        match res {
          Ok(_)=>{println!("Write success!!!")}
          Err(error)=>{println!("write file error occurred!! {:?}",error)}
        }

      },
      Err(error) => {
        println!("get gfwList Error {:?}",error)
      },
  }

}

  fn processLine(result:&mut String,target:&str,dns:&str,ipset:&str){ 
             let dns_line = format!("server=/{}/{}\n",target,dns);
               result.push_str(&dns_line);
             let ipset_line = format!("ipset=/{}/{}\n",target,ipset); 
               result.push_str(&ipset_line); 
  }

fn get_gfwlist()-> Result<String, Box<dyn std::error::Error>> {

     let proxy = Proxy::https("http://127.0.0.1:8001").unwrap();
     let http_client = reqwest::blocking::ClientBuilder::new().proxy(proxy).build()?;
     let req = http_client.get("https://raw.githubusercontent.com/gfwlist/gfwlist/master/gfwlist.txt").build()?;
    let res = http_client.execute(req)?.text()?.replace("\n", "");

                  let decoded = base64::decode_config(res, base64::Config::new(base64::CharacterSet::Standard, false))?;
                        let text = String::from_utf8(decoded)?;
                                   println!("we got decode content {:?}",text.lines().count());
                                   Ok(text)



}


fn write_file(content:String, file: &str) ->Result<(),Error>{ 
    let count = fs::write(file, content)?;
    Ok(()) 
}

fn testServer(){
  let listner = net::TcpListener::bind("127.0.0.1:8088").unwrap();
   let pool = thread_pool::ThreadPool::new(4);
  for stream in listner.incoming(){

    
      println!("we got a request ");
    if let Ok(mut tcp_stream) = stream{

      pool.execute(move||{


        let addr = tcp_stream.local_addr().unwrap().to_string();
      println!("we got a tcp_stream from {} ",addr);
      let mut buffer = vec![0;512];
      let size = tcp_stream.read(&mut buffer).unwrap();
      let content = String::from_utf8_lossy(&buffer);




      //let response = format!("HTTP/1.1 200 OK\r\n\r\nWe Got {} bytes, and the request content is2{}",buffer.len(),content);
      let mut buf = vec![0u8;512];
      let mut file = File::open("hello.html").unwrap();
      let mut allCount = fs::metadata("hello.html").unwrap().len();
      let response = format!("HTTP/1.1 200 OK\r\nContent-Type:text/html; charset=UTF-8\r\ncontent-length: {}\r\n\r\n",allCount);
      println!("response ====={}",response);
       tcp_stream.write_all(response.as_bytes()).unwrap();
      loop{
       let count = file.read(&mut buf).unwrap();
       tcp_stream.write(&mut buf[..count]).unwrap();
       //tcp_stream.write_all(&mut buf).unwrap();
       if count < buf.len(){
         tcp_stream.flush().unwrap();
         break;
       }
      }



      // }





      println!("response success");


      });


    }else {

      println!("we got a error ");


    }



  }


}



fn main() {

//  let server_thread =  thread::spawn(||{

//   testServer();

//   });
   gfwlist2Dnsmasq();



//   server_thread.join().unwrap();

  // println!("main start ");

  // println!("main test end ");
  // thread::sleep(Duration::from_secs(5));

  // println!("main main end ");

//   let name = String::from("Hello1");
//   let b = name.clone();
//   let mut num = MyBox{
//     label:name,
//     left:5,
//     right:10,
//     top:12,
//     bottom: 28 
//   };
//   println!("name {}",b);

// let r1 = &num as *const MyBox;
// let r2 = &mut num as *mut MyBox;
// unsafe {

//   (*r2).right = 100;
//   (*r2).top= 100;
//   (*r2).left= 100;
//   (*r2).bottom= 100;
//   println!("mybox {:?}",(*r1));

// }
//   println!("mybox {:?}",num);
  // let args: Vec<String> = env::args().collect();
  // let args_os = env::args_os();
  // let mut vars = env::vars();
  // for (key,value) in vars{ 
  //   println!("{}：{}",key,value); 
  // }
//   let varsOs = env::vars_os();
//     println!("====================================="); 
//   for (key,value) in varsOs{ 
//     eprintln!("{}：{}",key.to_str().unwrap_or("??????????"),value.to_str().unwrap_or("=========")); 
//   }
//   let dir = env::current_dir();
//   let exe = env::current_exe();
//   let tmp_dir =  env::temp_dir();

//   let arch= env::consts::ARCH;
//   let os = env::consts::OS;
//   let family = env::consts::FAMILY;
//   let dllPrefix= env::consts::DLL_PREFIX;
//   let dllSuffix= env::consts::DLL_SUFFIX;
//   let dllExt= env::consts::DLL_EXTENSION;
//   let exeSuffix = env::consts::EXE_SUFFIX;
//   let exeExt = env::consts::EXE_EXTENSION;

//   let vecs = vec![1,2,3,4,5,6,7];
//   vecs.len();
//      let maaaap:Vec<String> = vecs.iter().map(|item|  format!("item {}",item) ).collect();

//      println!("mapp count {}",maaaap.len());

//     let count = vars.by_ref().count();

  
//     vars.by_ref().for_each(|(key,value)|{
//         println!("{}={}",key,value);
//     });
//     vars.by_ref().for_each(
// |(key,value)|{
//         println!("{}={}",key,value);
//     }
//     );

//     println!("count {}",count);



  // let home_dir = env::home_dir().unwrap();
  //   println!("os={:?} dir={:?} exe={:?} home={:?} ",os,dir,exe,home_dir);
  // let mut index =0;
  // for item in args{


  //   println!("The {} arg is {:?}",index,item);
  //   index +=1;

  // }
 // let content = readTxt("poem.txt");

  // let aaa = readBufferd("poem.txt22").unwrap_or_else(|err| {
  //       println!("Problem parsing arguments: {}", err);
  //       "ffffffffffffffffffffffff".to_string()
  //   });
  // println!("readbuffer {}",aaa);

  // match content{
  //    Ok(res)=>{

  // println!("file content is \n{}",res);
  // let third = get_third_word(&res);
  // println!("the third is {}", third );
  
  //   }
  //   Err(err)=>{ 
  //     println!("error {:?}", err.kind());


  //   } 
  // }
  //test_std_in_out();
    // let x = 5;
    // let y = &x;
    // let z = *y;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // println!("y:{}",*y);
    // println!("y:{:?}",y);
    // let t1 = MyBox{top:15,left:15,right:60,bottom:75};

    // let t2 = Rc::new(t1);
    // let t3 = Rc::clone(&t2);
    // let t4 = Rc::clone(&t2);
    // let t5 = Rc::clone(&t2);
    // let t6 = t2.clone();
    // let ref_count = Rc::strong_count(&t2);
    // println!("refcount {}",ref_count);
//        let v = vec![1, 2, 3];
//          let (tx, rx) = channel();
//     let tx1 = mpsc::Sender::clone(&tx);
//     let thread1 = thread::spawn(move||{
      
//         for i in 1..10{

//           thread::sleep(Duration::from_secs(1));
//           println!("msg send {}",i);
//           tx.send(format!("this is from sub thread1 {}",i));
//         };
//           thread::sleep(Duration::from_secs(10));
//           tx.send("this is the last msg1".to_string()); 
//     });
//     let thread2 = thread::spawn(move||{
      
//         for i in 1..10{

//           thread::sleep(Duration::from_secs(1));
//           println!("msg send {}",i);
//           tx1.send(format!("this is from sub thread2 {}",i));
//         };
//           thread::sleep(Duration::from_secs(5));
//           tx1.send("this is the last msg2".to_string()); 
//     });
// //     let res = rx.recv();
// //     let content = res.unwrap_or_else(|err|{ 
// //       println!("vector {:?}",err);
// //        "error occured".to_string()
// // });
// for res in rx {


//       println!("main thread: \"{:?}\"",res);

// }

  //   let counter = Arc::new( Mutex::new(0));
  //   let mut handles = vec![];

  //   for _ in 0..10 {
  //     let item = Arc::clone(&counter);
  //     //   let counter = Arc::clone(&counter);
  //       let handle = thread::spawn(move || {
  //           let mut num =item.lock().unwrap();

  //           *num += 1;
  //       });
  //       handles.push(handle);
  //   }

  //   for handle in handles {
  //       handle.join().unwrap();
  //   }

  //   println!("Result: {}", *counter.lock().unwrap());

  // //  thread1.join();
  // let shap1 = Sharp::Rectangle;
  // if let Sharp::Rectangle = shap1{


  // }


}

enum Sharp {
  Rectangle =1,
  Triangle =3


}


struct MyBox{
  label:String,
  top:i32,
  left:i32,
  right:i32,
  bottom:i32 
}

trait tangle{

  fn add(&self,x:i32,y:i32)->i32{
    x+y
  }

}

impl MyBox {
  fn circumference(&self) ->u32{
    let height = (self.bottom - self.top).abs();
    let width = (self.right- self.left).abs(); 
    ((height + width)*2 )as u32 
  }
  fn area(&self) ->u32{
    let height = (self.bottom - self.top).abs();
    let width = (self.right- self.left).abs(); 
    (height * width) as u32 
  }

}
impl Debug for MyBox{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //todo!()
        f.debug_tuple("MMMMBOX")
        .field(&self.top)
        .field(&self.bottom)
        .field(&self.left)
        .field(&self.right)
        .finish()
    }
}
impl Display for MyBox{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(top:{}, left:{},right:{},bottom:{})", self.top, self.left,self.right,self.bottom)
    }
}

/// Adds one to the number given.
///
/// #  simulated_expensive_calculation
///
/// ```
/// let arg = 5;
/// let answer =  simulated_expensive_calculation(arg);
///
/// assert_eq!(6, answer);
/// ```
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn test_std_in_out()->Result<String,io::Error>{
  io::stdout().write_all("输入用户名:\n".as_bytes());
  let mut name= String::new();
  //io::stdin().read_line(&mut name);
  io::stdin().read_line(&mut name);
  io::stdout().write_all("输入密码:\n".as_bytes());
  let mut pwd = String::new();
  io::stdin().read_line(&mut pwd);
  let ff_name = format!("name:{}pwd:{}",name,pwd);
  println!("ff_aa {}",ff_name );
  Ok("success".to_string())
}

fn raw_back<'a>(input2:&'a str)->&'a str{
    input2
}

fn get_third_word(input : &str) ->&str{
    let mut start = 0;
    let mut end = input.chars().count();
    let mut spaceIndex = 0;
    let mut index = 0;
    for item in input.chars(){
      if item == '\n'{

        match spaceIndex{
          2 =>{start = index }
          3 =>{end = index;
            break;
          } 
          _ =>{} 

        } 
        spaceIndex +=1; 
      }

      index +=1; 
    } 
    &input[start..end]
}

fn readBufferd(filepath: &str) ->Result<String,Error>{


  let mut file = File::open(filepath)?;
  let bufferd = BufReader::new(file);
  let mut line_num = 0;
  for line in bufferd.lines(){
         println!("line {}: {}",line_num, line?);
      line_num +=1;
  }


  Ok("read success".to_string())

}


fn readTxt(filepath: &str) ->Result<String,Error>{

  let mut file = File::open(filepath)?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;
  Ok(content )
}