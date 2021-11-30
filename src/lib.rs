use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

impl DataPoint {
    pub fn new(x: f64, y: f64) -> Self {
        DataPoint { x, y }
    }
}

pub struct ImbueContext {
    pub dataset: Vec<DataPoint>,
    pub total_count: usize,
    pub imbue_count: usize,
    pub axis_min: f64,
    pub axis_max: f64,
}

impl ImbueContext {
    pub fn new(dataset: Vec<DataPoint>) -> Self {
        let (axis_min, axis_max) = ImbueContext::axis_min_and_max(&dataset);
        let total_count = ((axis_max - axis_min).abs() + 1.0) as usize;
        let imbue_count = total_count - dataset.len();
        ImbueContext {
            dataset,
            axis_min,
            axis_max,
            total_count,
            imbue_count,
        }
    }

    pub fn axis_range(&self) -> RangeInclusive<i64> {
        self.axis_min as i64..=self.axis_max as i64
    }

    pub fn known_x(&self) -> HashSet<i64> {
        self.dataset.iter().map(|data| data.x as i64).collect()
    }

    fn axis_min_and_max(dataset: &Vec<DataPoint>) -> (f64, f64) {
        return dataset
            .iter()
            .map(|data| data.x)
            .fold((f64::MAX, f64::MIN), min_and_max);
    }
}

fn min_and_max(mut accumulator: (f64, f64), item: f64) -> (f64, f64) {
    if item < accumulator.0 {
        accumulator.0 = item;
    }
    if item > accumulator.1 {
        accumulator.1 = item
    }

    accumulator
}

pub fn average(context: &ImbueContext) -> Vec<DataPoint> {
    let imbue_count = context.imbue_count;
    if imbue_count == 0 {
        return vec![];
    }

    let mut sorted_dataset = context.dataset.clone();
    sorted_dataset.sort_by_key(|datapoint| datapoint.x as i64);

    sorted_dataset
        .windows(2)
        .filter(|window| window[0].x as i64 + 1 != window[1].x as i64)
        .map(|window| (window[0], window[1]))
        .flat_map(average_imbue_window)
        .collect()
}

fn average_imbue_window(window: (DataPoint, DataPoint)) -> Vec<DataPoint> {
    let start = window.0.x as i64 + 1;
    let end = window.1.x as i64 - 1;
    let missing_count = end - start + 1;

    // Add one to the missing count so that the last value calculated
    // is not equal to window.1.y
    let delta = (window.0.y - window.1.y).abs() / (missing_count + 1) as f64;
    let delta = if window.0.y > window.1.y {
        -delta
    } else {
        delta
    };

    let mut missing = Vec::with_capacity(missing_count as usize);
    let mut total_change = window.0.y + delta;
    for x in start..=end {
        missing.push(DataPoint::new(x as f64, total_change));
        total_change += delta;
    }
    missing
}

pub fn zeroed(context: &ImbueContext) -> Vec<DataPoint> {
    let imbue_count = context.imbue_count;
    if imbue_count == 0 {
        return vec![];
    }

    let known_x = context.known_x();
    return context
        .axis_range()
        .filter(|x| !known_x.contains(&x))
        .map(|x| DataPoint::new(x as f64, 0.0))
        .collect();
}

pub fn last_known(context: &ImbueContext) -> Vec<DataPoint> {
    let imbue_count = context.imbue_count;
    if imbue_count == 0 {
        return vec![];
    }

    let dataset_map = dataset_map(&context.dataset);
    let mut imbued_dataset = Vec::with_capacity(imbue_count);
    let mut last_known = 0.0;
    for x in context.axis_range() {
        if dataset_map.contains_key(&x) {
            last_known = dataset_map.get(&x).unwrap().clone();
        } else {
            imbued_dataset.push(DataPoint::new(x as f64, last_known))
        }
    }

    imbued_dataset
}

fn dataset_map(dataset: &Vec<DataPoint>) -> HashMap<i64, f64> {
    dataset.iter().map(|data| (data.x as i64, data.y)).collect()
}

#[cfg(test)]
mod tests {
    use crate::{DataPoint, ImbueContext};

    #[test]
    fn test_average_imbue() {
        let dataset = vec![DataPoint::new(1.0, 123.0), DataPoint::new(5.0, 43.0)];
        let context = ImbueContext::new(dataset);
        let imbued_dataset = crate::average(&context);

        let expected_dataset: Vec<DataPoint> = vec![
            DataPoint::new(2.0, 103.0),
            DataPoint::new(3.0, 83.0),
            DataPoint::new(4.0, 63.0),
        ];
        assert_eq!(imbued_dataset, expected_dataset);
    }

    #[test]
    fn test_average_imbue_with_multiple_consecutive_missing_groups() {
        let dataset = vec![
            DataPoint::new(1.0, 123.0),
            DataPoint::new(5.0, 43.0),
            DataPoint::new(8.0, 80.0),
        ];
        let context = ImbueContext::new(dataset);
        let imbued_dataset = crate::average(&context);

        let expected_dataset: Vec<DataPoint> = vec![
            DataPoint::new(2.0, 103.0),
            DataPoint::new(3.0, 83.0),
            DataPoint::new(4.0, 63.0),
            DataPoint::new(6.0, 55.333333333333336),
            DataPoint::new(7.0, 67.66666666666667),
        ];
        assert_eq!(imbued_dataset, expected_dataset);
    }

    #[test]
    fn test_average_imbue_with_flat_average() {
        let dataset = vec![DataPoint::new(1.0, 123.0), DataPoint::new(5.0, 123.0)];
        let context = ImbueContext::new(dataset);
        let imbued_dataset = crate::average(&context);

        let expected_dataset: Vec<DataPoint> = vec![
            DataPoint::new(2.0, 123.0),
            DataPoint::new(3.0, 123.0),
            DataPoint::new(4.0, 123.0),
        ];
        assert_eq!(imbued_dataset, expected_dataset);
    }

    #[test]
    fn test_zeroed_imbue() {
        let dataset = vec![DataPoint::new(1.0, 123.0), DataPoint::new(5.0, 43.0)];
        let context = ImbueContext::new(dataset);
        let imbued_dataset = crate::zeroed(&context);

        let expected_dataset = vec![
            DataPoint::new(2.0, 0.0),
            DataPoint::new(3.0, 0.0),
            DataPoint::new(4.0, 0.0),
        ];
        assert_eq!(imbued_dataset, expected_dataset);
    }

    #[test]
    fn test_zeroed_imbue_with_negative_values() {
        let dataset = vec![
            DataPoint::new(-5.0, 67.0),
            DataPoint::new(1.0, 123.0),
            DataPoint::new(5.0, 43.0),
        ];
        let context = ImbueContext::new(dataset);
        let imbued_dataset = crate::zeroed(&context);

        let expected_dataset = vec![
            DataPoint::new(-4.0, 0.0),
            DataPoint::new(-3.0, 0.0),
            DataPoint::new(-2.0, 0.0),
            DataPoint::new(-1.0, 0.0),
            DataPoint::new(0.0, 0.0),
            DataPoint::new(2.0, 0.0),
            DataPoint::new(3.0, 0.0),
            DataPoint::new(4.0, 0.0),
        ];

        assert_eq!(imbued_dataset, expected_dataset);
    }

    #[test]
    fn test_last_known_imbue() {
        let dataset = vec![
            DataPoint::new(7.0, 84.0),
            DataPoint::new(1.0, 123.0),
            DataPoint::new(4.0, 56.0),
        ];
        let context = ImbueContext::new(dataset);
        let imbued_dataset = crate::last_known(&context);

        let expected_dataset: Vec<DataPoint> = vec![
            DataPoint::new(2.0, 123.0),
            DataPoint::new(3.0, 123.0),
            DataPoint::new(5.0, 56.0),
            DataPoint::new(6.0, 56.0),
        ];
        assert_eq!(imbued_dataset, expected_dataset);
    }

    #[test]
    fn test_last_known_imbue_with_negative_values() {
        let dataset = vec![
            DataPoint::new(-2.0, 50.5),
            DataPoint::new(1.0, 123.0),
            DataPoint::new(4.0, 56.0),
            DataPoint::new(7.0, 84.0),
        ];
        let context = ImbueContext::new(dataset);
        let imbued_dataset = crate::last_known(&context);

        let expected_dataset: Vec<DataPoint> = vec![
            DataPoint::new(-1.0, 50.5),
            DataPoint::new(0.0, 50.5),
            DataPoint::new(2.0, 123.0),
            DataPoint::new(3.0, 123.0),
            DataPoint::new(5.0, 56.0),
            DataPoint::new(6.0, 56.0),
        ];
        assert_eq!(imbued_dataset, expected_dataset);
    }
}
