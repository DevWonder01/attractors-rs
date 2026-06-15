# Attractor Equations

This document contains the differential equations for all 11 strange attractors currently implemented in the 3D simulation.

### 1. Lorenz Attractor
$$ \frac{dx}{dt} = \sigma (y - x) $$
$$ \frac{dy}{dt} = x (\rho - z) - y $$
$$ \frac{dz}{dt} = x y - \beta z $$

### 2. Thomas' Cyclically Symmetric Attractor
$$ \frac{dx}{dt} = \sin(y) - b x $$
$$ \frac{dy}{dt} = \sin(z) - b y $$
$$ \frac{dz}{dt} = \sin(x) - b z $$

### 3. Chen Attractor
$$ \frac{dx}{dt} = a (y - x) $$
$$ \frac{dy}{dt} = (c - a) x - x z + c y $$
$$ \frac{dz}{dt} = x y - b z $$

### 4. Dadras Attractor
$$ \frac{dx}{dt} = y - a x + b y z $$
$$ \frac{dy}{dt} = c y - x z + z $$
$$ \frac{dz}{dt} = d x y - e z $$

### 5. Rössler Attractor
$$ \frac{dx}{dt} = -y - z $$
$$ \frac{dy}{dt} = x + a y $$
$$ \frac{dz}{dt} = b + z (x - c) $$

### 6. Halvorsen Attractor
$$ \frac{dx}{dt} = -a x - 4y - 4z - y^2 $$
$$ \frac{dy}{dt} = -a y - 4z - 4x - z^2 $$
$$ \frac{dz}{dt} = -a z - 4x - 4y - x^2 $$

### 7. Rabinovich-Fabrikant Attractor
$$ \frac{dx}{dt} = y (z - 1 + x^2) + \gamma x $$
$$ \frac{dy}{dt} = x (3z + 1 - x^2) + \gamma y $$
$$ \frac{dz}{dt} = -2z (\alpha + x y) $$

### 8. Three-Scroll Unified Chaotic System
$$ \frac{dx}{dt} = a (y - x) + c x z $$
$$ \frac{dy}{dt} = b x - x z + f y $$
$$ \frac{dz}{dt} = d z + x y - e x^2 $$

### 9. Sprott Attractor (Sprott LinZ)
$$ \frac{dx}{dt} = y + a x y + x z $$
$$ \frac{dy}{dt} = 1 - b x^2 + y z $$
$$ \frac{dz}{dt} = x - x^2 - y^2 $$

### 10. Four-Wing Attractor
$$ \frac{dx}{dt} = a x + y z $$
$$ \frac{dy}{dt} = b x + c y - x z $$
$$ \frac{dz}{dt} = -z - x y $$

### 11. Langford (Aizawa) Attractor
$$ \frac{dx}{dt} = (z - b)x - d y $$
$$ \frac{dy}{dt} = d x + (z - b)y $$
$$ \frac{dz}{dt} = c + a z - \frac{z^3}{3} - (x^2 + y^2)(1 + e z) + f z x^3 $$
