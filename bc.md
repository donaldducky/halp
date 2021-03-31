# bc

```bash
# To get 1 decimal precision
echo 'p=39308/269055*100; scale=1; p/1' | bc -l
14.6

# otherwise we get something like this...
echo 'scale=2; 39308/269055*100' | bc
14.00
echo 'scale=3; 39308/269055*100' | bc
14.600
```
