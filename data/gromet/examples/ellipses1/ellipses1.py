import numpy


X = numpy.reshape(numpy.arange(2*3*4), (2,3,4))
# array([[[ 0,  1,  2,  3],
#         [ 4,  5,  6,  7],
#         [ 8,  9, 10, 11]],
# 
#        [[12, 13, 14, 15],
#         [16, 17, 18, 19],
#         [20, 21, 22, 23]]])

Y = X[..., 0]
# array([[ 0,  4,  8],
#        [12, 16, 20]])

Z = X[0, ..., 0]
# array([0, 4, 8])