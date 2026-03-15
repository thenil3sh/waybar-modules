# Pomodoro Timer

<video controls>
  <source src="../assets/basic.mp4" type="video/quicktime">
  Your browser does not support the video tag.
</video>
<video controls>
  <source src="../assets/session_over.mp4" type="video/quicktime">
  Your browser does not support the video tag.
</video>


This crate provides two modules, 
- `doro_tx` - Sends signal to `doro_rx`. It doesn't care, checking if signal made it safe.

```shell
❮ doro_tx --help
Usage: doro_tx <COMMAND>

Commands:
  increase    Add minutes to timer (1 minute is added by default)
  decrease    Decreases timer minute by 1 (or n if specified)
  toggle      Toggles the view between Break & Session timer
  play-pause  Starts/Pauses/Continues timer tick
  reset       Sets timer to 00:00
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

- `doro_rx` - This is a listener, also what waybar runs as a module and reads its stdout pipeline.
```shell
❮ doro_rx --help
Usage: doro_rx [OPTIONS]

Options:
      --on-session-over <ON_SESSION_OVER>  Executes the command when session is over
      --on-break-over <ON_BREAK_OVER>      Executes the command when break is over
  -h, --help                               Print help
```

> [!TIP]
> Instead of using `doro_tx`, one can also connect to UNIX socket, present at : `/tmp/doro_timer.sock` (only available when `doro_rx` is alive)
> 
> Example : 
> ```bash
>   printf "<message>" | socat - UNIX-SENDTO:/tmp/doro_timer.sock
> ```
> _and ofcourse, replace `<message>` with one of the following :_
> 
> | Message | Action | 
> |--------|--------|
> | `in` | Increments timer by a minute |
> | `dc` | Decrements timer by a minute |
> | `tg` | Toggles between `session_timer` and `break_timer` |
> | `pl` | Plays/Pauses the active timer |
> | `rs` | Resets the visible timer to `00:00` |
> 
> Just fire a datagram, and disconnect; Socket for transmission isn't necessary here, neither a persistant connection is required. 

## Configuration
> [!NOTE]  
> You need [Nerd Font(s)](https://www.nerdfonts.com/font-downloads), if you plan to use this configuration. Otherwise you won't see any symbol typefaces.\
- `waybar/config.json` : 
```json
"custom/doro_timer" : {
    "exec" : "doro_rx",
    "return-type" : "json",
    "on-scroll-up" : "doro_tx increase",
    "on-scroll-down" : "doro_tx decrease",
    "on-click" : "doro_tx play-pause",
    "smooth-scrolling-threshold" : 1.5,
    "on-click-right" : "doro_tx toggle",
    "on-click-middle" : "doro_tx reset",
    "format" : "{icon} {text}",
    "hide-empty-text" : false,
    "align" : 0,
    "format-icons" : {
      "session-timer" : "󰥔",
      "break-timer" : "",
      "session-over" : "",
      "break-over" : ""
    }
  }
```

- `waybar/style.css`
```css
@define-color love #eb6f92;
@define-color rose #ebbcba;
@define-color bg_color #191724;


/* Base */
#custom-doro_timer {
  border-radius: 12px;
  padding : 0px 12px;
  transition : all 0.5s ease-out;
  transition : color 0.5s;
  background-color: @bg_color;
}

/* Session timer */
#custom-doro_timer.session-timer {
  color : alpha(@love, 0.7);
}
#custom-doro_timer.session-timer:hover {
  color : alpha(@love, 1);
}


/* Session timer (paused) */
#custom-doro_timer.session-timer.paused {
  color : alpha(@love, 0.5);
}
#custom-doro_timer.paused.session-timer:hover {
  color : alpha(@love, 0.7);
}

/* Session over */
#custom-doro_timer.session-over {
  transition : background-color 0.3s ease-out;
  border : 3px solid @bg_color;
  color : @foam;
}
#custom-doro_timer.session-over:hover {
  background: @foam;
  color : @bg_color;
}


/* Break Timer */
#custom-doro_timer.break-timer {
  color : @foam;
}

/* Break Timer (paused) */
#custom-doro_timer.break-timer.paused {
}

/* Break Over */
#custom-doro_timer.break-over {
  padding-right : 18px;
  padding-left : 24px;
  border-radius : 20px;
  border : 3px solid @bg_color;
  color : @iris;
  transition : background 0.3s ease-out;
}
#custom-doro_timer.break-over:hover {
  color : @bg_color;
  background : @love;
}
```
