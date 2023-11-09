use safe_drive::msg::common_interfaces::{sensor_msgs, geometry_msgs::msg::Point32};



pub fn nearest_neighbor_association(
    previous_point: &[Point32], 
    current_point: &[Point32]
)
{
    let mut delta_points = vec![
        (previous_point.get(0).unwrap().x - current_point.get(0).unwrap().x,
        previous_point.get(0).unwrap().y - current_point.get(0).unwrap().y,
        previous_point.get(0).unwrap().z - current_point.get(0).unwrap().z)];

    for i in 1..current_point.len()
    {
        delta_points.push((
            previous_point.get(i).unwrap().x - current_point.get(i).unwrap().x,
            previous_point.get(i).unwrap().y - current_point.get(i).unwrap().y,
            previous_point.get(i).unwrap().z - current_point.get(i).unwrap().z,
        ))
    }

    let mut d = 0.0;

    for i in 0..delta_points.len()
    {
        
    }
}