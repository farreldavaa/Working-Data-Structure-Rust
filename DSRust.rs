#[derive(Debug)]
struct Tuples_<'a>{
    color : (&'a str, &'a str, &'a str, &'a str,&'a str,&'a str,&'a str,&'a str,&'a str,&'a str)
    // color1 : (&'a str, &'a str, &'a str, &'a str),
    // color2 : (&'a str, &'a str, &'a str, &'a str)
}

fn str(text: &str){
    println!("{:?}", text)
}

fn str_str(){
    let input = String::from("Farrel");
    str(&input);
}

fn str_string(){
    let convert = str_str().to_owned();
    println!("{:?}", convert);
}

fn vector(){
    let mut vec_contain = Vec::new();
    vec_contain.push(1);
    vec_contain.push(2);
    vec_contain.push(6);
    vec_contain.push(5);
    vec_contain.push(7);
    vec_contain.push(9);
    vec_contain.push(13);
    vec_contain.push(21);
    vec_contain.push(23);
    vec_contain.push(31);
    for prima in vec_contain.iter(){
        // DRAFT //
        // if  10 % 5 == 0{
        //     println!("{} adalah bilangan prima pada Vector", prima);
        // } else {
        //     break
        // }
        if prima > &1 {
            let mut is_prima = true;
            for a in 2..prima-1 {
                if prima % a == 0 {
                    is_prima = false;
                    break;
                }
            }
            if is_prima {
                println!("{} adalah bilangan prima pada Vector", prima);
            }
        } else {
            continue
        }
    }
}

fn tuple(){
    let color = ("r:255, g:0, b:0, a:0","r:240, g:248, b:255, a:1",
    "r:245, b:245, g:220, a:1", "r:255, g:235, b:205, a:1", "r:0, b:0, g:0, a:1",
    "r:0, b:0, g:255, a:1", "r:220, g:20, b:60, a:1", "r:0, g:139, b:139, a:1",
    "r:76, g:61, b:139, a:1","r:255, g:20, b:147, a:1");
    let tuple = Tuples_{
        color,
        // color1,
        // color2
    };
    let destructA = tuple.color.0;
    let destructB = tuple.color.7;
    let destructC = tuple.color.9;
    println!("{:?}", destructA);
    println!("{:?}", destructB);
    println!("{:?}", destructC);
    // println!("b: {}",b);
    // println!("g: {}",g);
    // println!("a: {}",a);

    // let color = ("r:250", "g:235", "b:215", "a:1");
    // let rgba = Tuples_<'a>{
        // while color == true
        // println!("{:?}",rgba);
    // }
    // let printinit = rgba.color.2;
    // println!("{:?}",printinit);
}

fn main(){
    println!("Nomor 1");
    str_string();
    println!("Nomor 2");
    vector();
    println!("Nomor 3");
    tuple();
}

