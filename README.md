Uses brightnessctl to get screen brightness and pamixer to get volume from pulseaudio. I will not be adding another way to get the volume so do it yourself if you don't use pulse.

I think I uploaded all the right things so you can just clone it and compile it.

Also make sure to edit /src/main.rs to configure the bar to have the right things in it. At the moment it just has my setting which shows the date and time, current audio volume, cpu usage, and battery percentage. In addition to those four it can display screen brightness and uptime at the moment. 
