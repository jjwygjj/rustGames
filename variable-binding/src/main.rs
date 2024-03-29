fn main() {
    //这一节讲变量绑定，根据这一节重点包括:变量绑定，模式，类型注解，可变性，初始化绑定，作用域和隐藏
    //根据这些特性编写“抢板凳游戏”。
    //游戏介绍:有3种颜色板凳红黄蓝，和三种颜色卡片红黄蓝黑，卡片上有技能包括:调包，锁定
    //游戏规则:三个人玩游戏分别坐在一个颜色上，把卡片当到中间，每人初始4张牌，每人轮流摸2张牌再必须打出1张牌，有人打出一张牌后，顺时针依次问是否要这张牌，如果要就将这张牌
    //放到板凳上，否则问下一个人 如果都不要这张牌回到发出者手里，输赢判断 当自己凳子上的牌有三张并且颜色和自己凳子颜色相同则获胜，如果有人收集了3张黑色卡片则死亡。
    //技能介绍，调包卡牌：可以在卡牌传输过程中进行改变替换，锁定卡牌：当被指定为锁定当卡牌到你面前必须接收
    //开始游戏go！

    //游戏开始由红色玩家打出一张卡牌(rust 默认变量都是不可变的，但是现在游戏初始化的卡牌都是可变的)
    let mut card: &str = "一张黑色卡牌";//可变性,初始化绑定
    let (card1, card2) = ("一张蓝色(调包)卡牌", "一张红色(锁定)卡牌");//模式，可变性,初始化绑定
    println!("红色玩家打出：{:?}", card);
    //这时候红色玩家是个老手他知道自己传的是黑色卡牌，以防万一每人要这张牌回到自己这里他对蓝色玩家使用 锁定技能,红色玩家结束回合。
    //黄色玩家保持警惕，不要此卡牌，黄色玩家回合结束。
    println!("黄色玩家感觉这是：{:?}", card);
    //到蓝色玩家的时候，锁定技能发动了，可是黄色玩家想英雄救美(也许蓝色玩家是个mm),打出一张调包卡牌
    println!("黄色玩家打出：{:?}", card1);
    card = "一张蓝色卡牌";//可变性
    println!("蓝色玩家正在犹豫是否要这张牌，：{:?}", card);
    //此时由红色玩家发动的锁定技能生效蓝色玩家必须要这张卡牌
    println!("红色玩家锁定卡牌生效，：{:?}", card2);
    let card = card;//作用域和隐藏(此时card变为不可变)
    println!("蓝色玩家要这，：{:?}", card);
    //愉快的玩耍吧。

    //本节主要是rust的特点，变量绑定let,类型注解@str,初始化绑定和模式为代码前2行，可变性当有关键字mut时候此变量可变，默认为不可变，
    //let card = card 隐藏允许我们重绑定一个值为不同的类型。它也可以改变一个绑定的可变性


}
