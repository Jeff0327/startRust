fn main(){

    let x=5u32;
    //u32 값으로 생성
    let y={
        let x_squerd=x*x;
        let x_cube=x_squerd*x;

        x_cube+x_squerd+x_cube
        //리턴값을 위해 세미콜론을 생략한다
    };

    let z={
        2*x;
        //세미콜론이 있을경우 리턴값이 없음 () 반환
    };

    println!("x is{:?}",x);
    println!("y is{:?}",y);
    println!("z is{:?}",z);

}

// struct Person{
//     name:String,
//     age:u32,
// }
//구조체

const MAX_POINTS:u32 = 100_000;
//아래 하이픈은 컴파일에서 읽지않고 순수 사람이 숫자를 구분하기편하게해줌
//변수 선언시 사용여부에 따라서 앞에써준다 예)_MAX_POINTS 사용되는부분이 없을때 => 메모리변수할당이안됨
//사용될때 => MAX_POINTS println!("{},MAX_POINTS")
