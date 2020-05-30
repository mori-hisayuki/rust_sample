fn main() {

    {
        let hello = "Hello";
        println!("{}",hello);
    }

    {
        let x= five();

        let y= plus_one(3);

        another_function(x, y);

        println!("y = {}",y);
    }

    {
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }

        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFT OFF!!!");
    }

    // 変数スコープ
    // String型
    {
        let mut s1 = String::from("hello");
        s1.push_str(", world!");
        let s2 = s1;
        println!("{}", s2);
    }

    // 変数のMOVE
    // ここからスコープ開始
    {
        // ここではs1という所有者
        let s1 = "hello";
        // ここで所有者がs2に移動して、s1は所有者でなくなる
        let s2 = s1;

        // s1とs2を呼び出しているが、
        // s1は既に所有者ではないため
        // そのためコンパイルエラー
        println!("s1 = {}, s2 = {}", s1, s2);
    }
    // このスコープを抜けたらs1もs2もメモリ解放する

    // Stuck:i8
    {
        let x:i8 = 5;
        let y:i8 = x;

        println!("The value of x is:i8 {}", x);
        println!("The value of y is:i8 {}", y);
    }
    // Stuck:i8
    {
        let x:f32 = 5.5;
        let y:f32 = x + 4.3;

        println!("The value of x is:i8 {}", x);
        println!("The value of y is:i8 {}", y);
    }
    // Stuck:str
    {
        let x:&str = "sample";
        let y:&str = x;

        println!("The value of x is:str {}", &x[..]);
        println!("The value of y is:str {}", y);
    }
    // Stuck:isize
    {
        let x: isize = 300;
        let y = x;

        println!("The value of x is:isize {}", x);
        println!("The value of y is:isize {}", y);
    }
    // Stuck:bool
    {
        let x: bool = true;
        let y = !x;

        println!("The value of x is:isize {}", x);
        println!("The value of y is:isize {}", y);
    }
    // Stuck:String
    {
        let x: (i32, String) = (1, "hello".to_string());
        let y:  (i32, String) = x;

        println!("The value of x is:(i32, String) ({},{})", y.0,y.1);
    }


    {
        let s = String::from("hello");  // sがスコープに入る
        takes_ownership(s);     // sの値が関数にムーブされ...
                                            // ... ここではもう有効ではない

        let x = 5;                    // xがスコープに入る

        makes_copy(x);          // xも関数にムーブされるが、
                                            // i32はCopyなので、この後にxを使っても大丈夫
    }


}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
