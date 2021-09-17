## hit_sphere calculation

hit_sphere calculates whether or not a ray from the camera origin
will hit a sphere with the given center and radius

P(t): the ray from the camera to the plane
C: the center of the sphere
r: the sphere's radius

(P(t)−C)⋅(P(t)−C) = r^2
gives
(A+t*B−C)⋅(A+t*B−C) = r^2
where
P(t) = A+t*B
expanded
t^2*B⋅B + 2t*B⋅(A−C) + (A−C)⋅(A−C) − r^2 = 0
where * is scalar multiplication and ⋅ is the dot product

discriminant
for a'x^2 + b'x + c'
the discriminant is b'^2 - 4a'c'
where
a' = B⋅B
b' = 2*B⋅(A−C)
c' = (A−C)⋅(A−C) − r^2

simplification
(-b +/- (b^2 - 4ac) ^ (1/2)) / 2a
substituting
b' = 2*h
gives
(-h +/- (h^2 - ac) ^ (1/2)) / a
where h = b⋅(A−C)
giving discriminant
(h^2 - ac)
