# MiFetch
## Mini fetch written in ~27 lines of code. Made just for fun
if you want to make custom ASCII, then you need open/create `/etc/mifetch/ascii.txt` and make ascii.
To colorize in start every line add:

`$1` - white 

`$2` - red

`$3` - green

`$4` - blue

`$5` - cyan

`$6` - yellow

`$7` - purple
to add custom colors, you can add to `colors` (in 3 line) your colors.
```
                       :====:       _-
                      -******=  ..:-++      
                      -******=   ;***=      
                       :====:      =*-      
                                          
                                            
*********:         +*******-.:=+*#%#        
%%%%%%%%%%-       *%#%%%%#%-=%%%###%*       
##########%-     *%#######%- *######%=      
###########%-  .*%########%- .#######%:     
############%=.#%#########%-  :%#######.    
##########################%-   -%#####%*    
##########################%-    +%#####%+   
##########################%-     *######%-  
##########%########*######%-     .#######%: 
########*.*%%%%%%%-:%#####%-      :%#######.
#########  ------: -%#####%-       =%#%%###=
#########          -%#####%-        **+-:''  

CPU: Intel(R) Core(TM) i7-4810MQ CPU @ 2.80GHz
OS: Arch Linux
RAM: 15.49GB
USED RAM: 5.17GB
SWAP: 7.75GB
HOST: miviodev
```
# Install
to install you need `cargo`:
```bash
cargo install mifetch
```
# build
```bash
git clone https://gitea.com/miviodev/mifetch.git
cd mifetch
cargo build -r
```
binary in `target/release/
