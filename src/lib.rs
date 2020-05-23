pub mod conversions
{
   pub mod temperature
    {
       pub fn c_to_f(c:f64) -> f64 
        {
            let f = c*(9.0/5.0) + 32.0;
            f
        }
        pub fn f_to_c(f: f64) -> f64
        {
            let c = (f-32.0)/1.8;
            c
        }
    }

}
