fn main() {
    let message = "To: jimb \r\nFrom: id \r\n\r\n Ooooooh, donuts!!\r\n";// この文字列は間を詰めておかないとちゃんと動かない。

    let mut lines = message.lines();
    println!("Headers:");
    //by_refでヘッダ部分のイテレータ取得
    for header in lines.by_ref().take_while(|l|!l.is_empty()){
        // is_emptyではなく
        println!("{}",header)
    }
    // 借用が返ってきたらもう一度イテレータとして使える
    // イテレータはヘッダ部分が消費された残り
    println!("\n Body:");
    for body in lines/*.by_ref()とかできる*/{
        println!("{}",body);
    }
}
