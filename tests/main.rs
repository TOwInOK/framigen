#[cfg(test)]
mod tests {

    use framigen::alignment::{Alignment, Top};
    use ril::{Border, Ellipse, Image, Line, Paste, Rectangle, Rgb};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn black_image() -> ril::Result<()> {
        let image = Image::new(512, 512, Rgb::black());
        image.save(ril::ImageFormat::Png, "sample.png")?;
        Ok(())
    }

    // #[test]
    // fn black_image_with_white_circle() -> ril::Result<()> {
    //     let mut image = Image::new(512, 512, Rgb::black());
    //     let tl = Alignment::Top(Top::LeftTop).directional((512, 512));
    //     let tl = Rectangle::at(tl.0, tl.1)
    //         .with_border(Border::new(Rgb::white(), 1))
    //         .with_size(efp, efp)
    //         .with_fill(Rgb::new(85, 85, 85));
    //     let tc = Rectangle::at(1 + efp, 1)
    //         .with_border(Border::new(Rgb::white(), 1))
    //         .with_size(efp, efp)
    //         .with_fill(Rgb::new(32, 42, 51));
    //     let tr = Rectangle::at(1 + efp + efp, 1)
    //         .with_border(Border::new(Rgb::white(), 1))
    //         .with_size(efp, efp)
    //         .with_fill(Rgb::new(100, 142, 151));

    //     let сl = Rectangle::at(1, 1 + efp)
    //         .with_border(Border::new(Rgb::white(), 1))
    //         .with_size(efp, efp)
    //         .with_fill(Rgb::new(85, 85, 85));
    //     let сc = Rectangle::at(1 + efp, 1 + efp)
    //         .with_border(Border::new(Rgb::white(), 1))
    //         .with_size(efp, efp)
    //         .with_fill(Rgb::new(32, 42, 51));
    //     let сr = Rectangle::at(1 + efp + efp, 1 + efp)
    //         .with_border(Border::new(Rgb::white(), 1))
    //         .with_size(efp, efp)
    //         .with_fill(Rgb::new(100, 142, 151));

    //     let bl = Rectangle::at(1, 1 + efp + efp)
    //         .with_border(Border::new(Rgb::white(), 1))
    //         .with_size(efp, efp)
    //         .with_fill(Rgb::new(85, 85, 85));
    //     let bc = Rectangle::at(1 + efp, 1 + efp + efp)
    //         .with_border(Border::new(Rgb::white(), 1))
    //         .with_size(efp, efp)
    //         .with_fill(Rgb::new(32, 42, 51));
    //     let br = Rectangle::at(1 + efp + efp, 1 + efp + efp)
    //         .with_border(Border::new(Rgb::white(), 1))
    //         .with_size(efp, efp)
    //         .with_fill(Rgb::new(100, 142, 151));
    //     {
    //         image.draw(&tl);
    //         image.draw(&tc);
    //         image.draw(&tr);

    //         image.draw(&сl);
    //         image.draw(&сc);
    //         image.draw(&сr);

    //         image.draw(&bl);
    //         image.draw(&bc);
    //         image.draw(&br);
    //     }

    //     image.save(ril::ImageFormat::Png, "sample.png")?;
    //     Ok(())
    // }
    macro_rules! rectangle {
        ($alig_dir:ident , $alig_size:expr; $($size:expr)?  $(, $fill:expr)? $(, ($border:expr, $thiknes:expr))?) => {{
            let size: (u32, u32) = alig_top!($alig_dir; $alig_size);
            Rectangle::at(size.0, size.1)
            $(.with_size($size.0, $size.1))?
            $(.with_fill($fill))?
            $(.with_border(Border::new($border, $thiknes)))?
        }};
    }
    macro_rules! alig_top {
        ($direction:ident; $size:expr) => {{
            Alignment::Top(Top::$direction).directional($size)
        }};
    }
    #[test]
    fn test_top() {
        let mut image = Image::new(512, 512, Rgb::black());

        let alignment = Alignment::Top(Top::LeftTop).directional((512, 512));

        let tl = rectangle!(LeftTop, (512, 512); (100, 100), Rgb::new(10,20,30), (Rgb::new(10,20,30), 1));
        // let top_left = Rectangle::at(alignment.0, tl.1)
        //     .with_border(Border::new(Rgb::white(), 1))
        //     .with_size(efp, efp)
        //     .with_fill(Rgb::new(85, 85, 85));
        // let top_center = Rectangle::at(1 + efp, 1)
        //     .with_border(Border::new(Rgb::white(), 1))
        //     .with_size(efp, efp)
        //     .with_fill(Rgb::new(32, 42, 51));
        // let top_right = Rectangle::at(1 + efp + efp, 1)
        //     .with_border(Border::new(Rgb::white(), 1))
        //     .with_size(efp, efp)
        //     .with_fill(Rgb::new(100, 142, 151));
        image.draw(&tl);
        image.save(ril::ImageFormat::Png, "sample.png").unwrap();
    }
}
