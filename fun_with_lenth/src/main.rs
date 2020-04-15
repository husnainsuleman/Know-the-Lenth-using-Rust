fn main() {

    let m=String::from("hello,");
       return_value(m)
  }
   
  
  
  fn return_value(s:String){
    let resutl=s.len();
   
    if resutl>5{
     let mut conter=0;
        loop{
           println!("Great value the 5 then -{},{}",resutl,s);
             conter+=1 ;       
              if conter ==3{
                break
        }    
  } 
   }
  else {
    let mut conter_1=0;
    loop{
      println!("Small the 5 {},{}",resutl,s);
       conter_1 +=1;
          if conter_1==5{
            break
  }
    }
  
  }
  
  }
   
   