use log::{debug};

pub fn closest_multiple(value: u16, multple: u16)->u16{

    let mut current = multple;

    if (value / current) == 0{
        debug!("closest multiple of {} for {}: {}", multple, value, 1);
        return 1;
    }

    while value / current !=0 {
        current+=multple;
    }

    debug!("closest multiple of {} for {}: {}", multple, value, current-multple);
    current-multple

}
