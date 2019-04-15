import "../css/styles.css";
import { startClock } from  "../crate/src/ts/clock";

(async () => {
    // we have to load wasm async
    // NOTE: files in crate/pkg/ exists ONLY after webpack compilation
    (await import('../crate/pkg/appname')).run();
    startClock();
})()
