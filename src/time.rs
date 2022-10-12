use psp::sys::*;


// float lastTick = getMilliseconds();
// float dt = 0;
// while ( window.isOpen() ) 
// {
// 	//inaccurate but often close enough
//     dt = (getMilliseconds() - lastTick);
//     lastTick = getMilliseconds();

// 	m_currState.input ( dt ); //dt for delta time
// 	m_currState.update ( dt );
// 	m_currState.draw ( dt );
// }

// double QuickGame_Timer_Delta(QGTimer* timer) {
//     u64 current;
//     sceRtcGetCurrentTick(&current);

// 	double dt = (double)(current - timer->last) / ((double)timer->resolution);
//     timer->total += dt;

//     timer->last = current;

//     return dt;
// }

