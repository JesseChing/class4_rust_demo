

fn main() {

    let square = Square{};
    area(&square);

    let circle = Circle{};
    area(&circle);

    let green_light = TrafficLight::Green;
    println!("green light time:{}", green_light.time());
    // assert_eq!(4, green_light.time());

    let nums1 = vec![1,2,3];
    print!("数组1:");
    for num in &nums1 {
        print!("{},", num);
    }
    println!("数组1相加后：{:?}", sum(&nums1));

    let nums2 = vec![u32::MAX,1];
    print!("数组2:");
    for num in &nums2 {
        print!("{},", num);
    }
    println!("数组2相加后：{:?}", sum(&nums2));
    
    // assert_eq!(Option::Some(6), sum(&nums1));
    // assert_eq!(Option::None, sum(&nums2));

}

 /*
 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
 */
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait TrafficTime {
    fn time(&self) -> u32;
}

impl TrafficTime for TrafficLight {
    fn time(&self) -> u32{
        match self {
            TrafficLight::Red => 3,
            TrafficLight::Green => 4,
            TrafficLight::Yellow => 5,
        }
    }
}

/* 
实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
 */
fn sum(nums:&[u32]) -> Option<u32>{
    let mut sum:u32 = 0;
    for elem in nums {
        if (sum + elem) < u32::MAX {
            sum += elem;
        } else {
            return  None;
        }
    }
    Some(sum)
}

/**
 * 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
 */
pub trait Shape {
    fn area(&self);
}

struct Circle{}

struct Triangle{}

struct Square{}

impl Shape for Circle {
    fn area(&self) {
        println!("圆形面积");
    }
}

impl Shape for Triangle {
    fn area(&self) {
        println!("三角形面积");
    }
}

impl Shape for Square {
    fn area(&self) {
        println!("正方形面积");
    }
}


fn area<T:Shape>(shape: &T){
    shape.area();
}