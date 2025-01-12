extern crate chrono;

use chrono::prelude::*;
extern crate base64;

use base64::encode;
use geo_svg::{Color, ToSvg};
use geo_types::Point;

fn main() {
    let now_time: DateTime<Local> = Local::now();
    let hour_now   = now_time.format("%H").to_string();
    let minute_now = now_time.format("%M").to_string();
    let second_now = now_time.format("%S").to_string();

    let column_one   = format!( "{0:04b}", hour_now[0..1].parse::<i8>().unwrap() );
    let column_two   = format!( "{0:04b}", hour_now[1..2].parse::<i8>().unwrap() );
    let column_three = format!( "{0:04b}", minute_now[0..1].parse::<i8>().unwrap() );
    let column_four  = format!( "{0:04b}", minute_now[1..2].parse::<i8>().unwrap() );
    let column_five  = format!( "{0:04b}", second_now[0..1].parse::<i8>().unwrap() );
    let column_six   = format!( "{0:04b}", second_now[1..2].parse::<i8>().unwrap() );

    let b1_1 = stroke_color( column_one[0..1].to_string() );
    let b1_2 = stroke_color( column_one[1..2].to_string() );
    let b1_3 = stroke_color( column_one[2..3].to_string() );
    let b1_4 = stroke_color( column_one[3..4].to_string() );

    let b2_1 = stroke_color( column_two[0..1].to_string() );
    let b2_2 = stroke_color( column_two[1..2].to_string() );
    let b2_3 = stroke_color( column_two[2..3].to_string() );
    let b2_4 = stroke_color( column_two[3..4].to_string() );

    let b3_1 = stroke_color( column_three[0..1].to_string() );
    let b3_2 = stroke_color( column_three[1..2].to_string() );
    let b3_3 = stroke_color( column_three[2..3].to_string() );
    let b3_4 = stroke_color( column_three[3..4].to_string() );

    let b4_1 = stroke_color( column_four[0..1].to_string() );
    let b4_2 = stroke_color( column_four[1..2].to_string() );
    let b4_3 = stroke_color( column_four[2..3].to_string() );
    let b4_4 = stroke_color( column_four[3..4].to_string() );

    let b5_1 = stroke_color( column_five[0..1].to_string() );
    let b5_2 = stroke_color( column_five[1..2].to_string() );
    let b5_3 = stroke_color( column_five[2..3].to_string() );
    let b5_4 = stroke_color( column_five[3..4].to_string() );

    let b6_1 = stroke_color( column_six[0..1].to_string() );
    let b6_2 = stroke_color( column_six[1..2].to_string() );
    let b6_3 = stroke_color( column_six[2..3].to_string() );
    let b6_4 = stroke_color( column_six[3..4].to_string() );

    let point = Point::new(3.0, 3.0);
    let point1_2 = Point::new(3.0, 9.0);
    let point1_3 = Point::new(3.0, 15.0);
    let point1_4 = Point::new(3.0, 21.0);

    let point2_1 = Point::new(11.0, 3.0);
    let point2_2 = Point::new(11.0, 9.0);
    let point2_3 = Point::new(11.0, 15.0);
    let point2_4 = Point::new(11.0, 21.0);

    let point3_1 = Point::new(22.0, 3.0);
    let point3_2 = Point::new(22.0, 9.0);
    let point3_3 = Point::new(22.0, 15.0);
    let point3_4 = Point::new(22.0, 21.0);

    let point4_1 = Point::new(30.0, 3.0);
    let point4_2 = Point::new(30.0, 9.0);
    let point4_3 = Point::new(30.0, 15.0);
    let point4_4 = Point::new(30.0, 21.0);

    let point5_1 = Point::new(41.0, 3.0);
    let point5_2 = Point::new(41.0, 9.0);
    let point5_3 = Point::new(41.0, 15.0);
    let point5_4 = Point::new(41.0, 21.0);

    let point6_1 = Point::new(49.0, 3.0);
    let point6_2 = Point::new(49.0, 9.0);
    let point6_3 = Point::new(49.0, 15.0);
    let point6_4 = Point::new(49.0, 21.0);

    let svg = point
        .to_svg()
        //   point1_1
        .with_radius(2.1)
        .with_stroke_color( Color::Named(b1_1) )
        .and(
            point1_2
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b1_2) ),
        )
        .and(
            point1_3
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b1_3) ),
        )
        .and(
            point1_4
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b1_4) ),
        )
        .and(
            point2_1
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b2_1) ),
        )
        .and(
            point2_2
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b2_2) ),
        )
        .and(
            point2_3
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b2_3) ),
        )
        .and(
            point2_4
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b2_4) ),
        )
        .and(
            point3_1
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b3_1) ),
        )
        .and(
            point3_2
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b3_2) ),
        )
        .and(
            point3_3
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b3_3) ),
        )
        .and(
            point3_4
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b3_4) ),
        )
        .and(
            point4_1
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b4_1) ),
        )
        .and(
            point4_2
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b4_2) ),
        )
        .and(
            point4_3
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color(Color::Named(b4_3)),
        )
        .and(
            point4_4
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b4_4) ),
        )
        .and(
            point5_1
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b5_1) ),
        )
        .and(
            point5_2
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b5_2) ),
        )
        .and(
            point5_3
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b5_3) ),
        )
        .and(
            point5_4
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b5_4) ),
        )
        .and(
            point6_1
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b6_1) ),
        )
        .and(
            point6_2
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b6_2) ),
        )
        .and(
            point6_3
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b6_3) ),
        )
        .and(
            point6_4
                .to_svg()
                .with_radius(2.1)
                .with_stroke_color( Color::Named(b6_4) ),
        )
        .with_stroke_width(1.3)
        .with_fill_opacity(0.);

    let data_svg = &svg.to_string();
    let base_64 = encode(data_svg);

    println!("| image={}", base_64);
}

fn stroke_color(num: String) -> &'static str {
    return if num == "0" {
        "gray"
    } else {
        "white"
    }
}
