

# a_fg -> i
# a_bg -> j
# r_fg -> k
# r_bg -> l

a = ((i / 255.0) + (j / 255.0) * (1.0 - (i / 255.0))) * 255

r = (((k / 255.0) * (i / 255.0)) + ((l / 255.0) * (j / 255.0) * (1.0 - (i / 255.0)))) / ((i / 255.0) + (j / 255.0) * (1.0 - (i / 255.0))) * 255



a = i + j - (i * j) / 255

r = (255 * k * i + 255 * j * l - i * j * l) / (65025(255 * i + 255 * j - ij))