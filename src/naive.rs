use std::ops::Sub;


// Don't like this. Interval doesn't generalize well for something like timestamps
pub fn down_sample_even_interval<U, V>(input: &[(U, V)], interval: usize) -> Vec<(U, V)> 
where
    for<'a> &'a U: Sub<&'a U, Output = U>,
    U: Clone + Sub<U, Output = U> + PartialOrd<usize>,
    V: Clone,
{
    let mut output = Vec::<(U, V)>::new();
    let mut last = input.iter().next().unwrap().clone();
    output.push(input.iter().next().unwrap().clone());
    for item in input {
        if (&item.0 - &last.0) > interval {
            last = item.clone();
            output.push(item.clone());
        }
    }
    return output;
}

// Faster if samples are evenly spaced we can skip computing the difference
pub fn down_sample_even_index<U, V>(input: &[(U, V)], interval: usize) -> Vec<(U, V)>
where
    U: Clone,
    V: Clone,
{
    let output = input.iter().step_by(interval).map(|x| x.clone()).collect();
    return output;
}