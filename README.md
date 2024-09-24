# bike_fitting
programms related to bikefitting concerns

# Stem length problem

![alt text](image.png)

use 
./stem_calculator <R> <s_a> <h_a> <h_b>

New stem formula
$$
s_b = \sqrt{(R+s_a)^2 + (h_a/2)^2 - (h_b/2)^2} - R
$$

## stem formula proof

To me "total reach" implies the distance between rider seat and hoods. I named it Reff. Bike  has frame reach R and stem length s.

Thus Reff_a is in relation with other variables as,

$$
R_{eff, a}^2 = (R+s_a)^2 + (h_a/2)^2
$$

and then for new bars and stem
$$
R_{eff, b}^2 = (R+s_b)^2 + (h_b/2)^2
$$

So the quadratic difference would be :
$$
R_{eff, b} - R_{eff, a} = \sqrt{(R+s_b)^2 + (h_b/2)^2} - \sqrt{(R+s_a)^2 + (h_a/2)^2}
$$

zeroing it would give :
$$
(R+s_b)^2 + (h_b/2)^2 = (R+s_a)^2 + (h_a/2)^2
$$

if h_b, h_a and s_a are kwown : I want to change handlebar and I change the stem accordingly :
$$
(R+s_b)^2 = (R+s_a)^2 + (h_a/2)^2 - (h_b/2)^2
$$

so,
$$
s_b = \sqrt{(R+s_a)^2 + (h_a/2)^2 - (h_b/2)^2} - R
$$