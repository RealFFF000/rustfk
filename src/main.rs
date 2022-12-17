use std::fs;
fn main() {
    let mut data = vec![0;1];
    let mut pointer = 0;
    let mut pointermax = 0;
    let mut iterations = 0;
    println!("Working...");
    let file = fs::read_to_string("input.txt")
        .expect("input file not found");
    let mut counter = 0;
    let mut programcounter = 0;
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
                programcounter +=1;
            }
            if character == '['{
                iterations = data[pointer]
            }
            else if character == ']'{
                iterations-=1;
                if iterations != 0{
                    counter = programcounter;        
                }
            }
            counter+=1;
            

    }
}
