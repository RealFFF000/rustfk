use std::fs;
pub fn petla(x:usize,y:usize){
    let mut pointer = x;
    let mut pointermax = x;
    let mut iterations = 0;
    let mut data = vec![0;1];
    let mut counter = y;
    let mut programcounter = 0;
    let file = fs::read_to_string("../input.txt")
    .expect("input file not found");
    loop{
            if file.chars().nth(counter) == None{
                break;
            }
            let  character = file.chars().nth(counter).unwrap();
            if character == '>'{
                pointer+=1;
            }
            else if character == '<'{
                pointer-=1;
            }
            if pointer > pointermax{
                pointermax = pointer;
                data.push(0);
            }
            if character == '+'{
                data[pointer] += 1;
            }
            else if character == '-'{
                data[pointer] -= 1;
            }
            if character == '.'{
                print!("{}", char::from(data[pointer]));
            }
            if iterations == 0{
                programcounter+=1;
            }
            if character == '['{
                petla(pointer,programcounter);
            }
            else if character == ']'{
                iterations-=1;
                if iterations == 0{
                    break;
                }else{
                    counter = programcounter;
                }
            }
            counter+=1;
            

    }
}


pub fn main(){
    petla(0,0);
}