# gpio
Ein Bibliothek für den STM32f3
([Dokumentation](https://drodofsky.github.io/gpio/gpio/index.html))

### Beispiel

```rust
PortC::init();
PortE::init();
let mut button = PortC::pin_in::<6>();
let mut led1 = PortE::pin_out::<9>();
let mut led2 = PortE::pin_out::<13>();
button.set_pull_down();
loop{
   if button.read() == 1 {
    	led1.set();
    	led2.reset();
	} else {
    	led1.reset();
    	led2.set();
	} 
}
```

