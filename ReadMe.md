
get the source and place it in ~/some/dir

create an other dir ~/some/otherdir

cd into it
[~/some/otherdir]> cmake ~/some/dir [optional stuff]
or
[~/some/otherdir]> cmake ../dir [optional stuff]


[optional stuff]

 - empty: build for current platform
 
 -DPLATFORM=ANDROID build for android
 