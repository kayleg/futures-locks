var searchIndex = {};
searchIndex["futures_locks"] = {"doc":"A library of [`Futures`]-aware locking primitives.  These locks can safely be  used in asynchronous environments like [`Tokio`].  When they block, they'll  only block a single task, not the entire reactor.","items":[[3,"Mutex","futures_locks","A Futures-aware Mutex.",null,null],[3,"MutexFut","","A `Future` representation a pending `Mutex` acquisition.",null,null],[3,"MutexGuard","","An RAII mutex guard, much like `std::sync::MutexGuard`.  The wrapped data can be accessed via its `Deref` and `DerefMut` implementations.",null,null],[3,"RwLock","","A Futures-aware RwLock.",null,null],[3,"RwLockReadFut","","A `Future` representation a pending `RwLock` shared acquisition.",null,null],[3,"RwLockWriteFut","","A `Future` representation a pending `RwLock` exclusive acquisition.",null,null],[3,"RwLockReadGuard","","An RAII guard, much like `std::sync::RwLockReadGuard`.  The wrapped data can be accessed via its `Deref` implementation.",null,null],[3,"RwLockWriteGuard","","An RAII guard, much like `std::sync::RwLockWriteGuard`.  The wrapped data can be accessed via its `Deref`  and `DerefMut` implementations.",null,null],[11,"drop","","",0,{"inputs":[{"name":"self"}],"output":null}],[11,"deref","","",0,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[11,"deref_mut","","",0,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[11,"poll","","",1,{"inputs":[{"name":"self"}],"output":{"name":"poll"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"mutex"}}],[11,"new","","Create a new `Mutex` in the unlocked state.",2,{"inputs":[{"name":"t"}],"output":{"name":"mutex"}}],[11,"try_unwrap","","Consumes the `Mutex` and returns the wrapped data.  If the `Mutex` still has multiple references (not necessarily locked), returns a copy of `self` instead.",2,{"inputs":[{"name":"self"}],"output":{"generics":["mutex"],"name":"result"}}],[11,"get_mut","","Returns a reference to the underlying data, if there are no other clones of the `Mutex`.",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"lock","","Acquires a `Mutex`, blocking the task in the meantime.  When the returned `Future` is ready, this task will have sole access to the protected data.",2,{"inputs":[{"name":"self"}],"output":{"name":"mutexfut"}}],[11,"try_lock","","Attempts to acquire the lock.",2,{"inputs":[{"name":"self"}],"output":{"generics":["mutexguard"],"name":"result"}}],[11,"deref","","",3,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[11,"drop","","",3,{"inputs":[{"name":"self"}],"output":null}],[11,"deref","","",4,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[11,"deref_mut","","",4,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[11,"drop","","",4,{"inputs":[{"name":"self"}],"output":null}],[11,"poll","","",5,{"inputs":[{"name":"self"}],"output":{"name":"poll"}}],[11,"poll","","",6,{"inputs":[{"name":"self"}],"output":{"name":"poll"}}],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",7,{"inputs":[{"name":"self"}],"output":{"name":"rwlock"}}],[11,"new","","Create a new `RwLock` in the unlocked state.",7,{"inputs":[{"name":"t"}],"output":{"name":"rwlock"}}],[11,"try_unwrap","","Consumes the `RwLock` and returns the wrapped data.  If the `RwLock` still has multiple references (not necessarily locked), returns a copy of `self` instead.",7,{"inputs":[{"name":"self"}],"output":{"generics":["rwlock"],"name":"result"}}],[11,"get_mut","","Returns a reference to the underlying data, if there are no other clones of the `RwLock`.",7,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"read","","Acquire the `RwLock` nonexclusively, read-only, blocking the task in the meantime.",7,{"inputs":[{"name":"self"}],"output":{"name":"rwlockreadfut"}}],[11,"write","","Acquire the `RwLock` exclusively, read-write, blocking the task in the meantime.",7,{"inputs":[{"name":"self"}],"output":{"name":"rwlockwritefut"}}],[11,"try_read","","Attempts to acquire the `RwLock` nonexclusively.",7,{"inputs":[{"name":"self"}],"output":{"generics":["rwlockreadguard"],"name":"result"}}],[11,"try_write","","Attempts to acquire the `RwLock` exclusively.",7,{"inputs":[{"name":"self"}],"output":{"generics":["rwlockwriteguard"],"name":"result"}}]],"paths":[[3,"MutexGuard"],[3,"MutexFut"],[3,"Mutex"],[3,"RwLockReadGuard"],[3,"RwLockWriteGuard"],[3,"RwLockReadFut"],[3,"RwLockWriteFut"],[3,"RwLock"]]};
initSearch(searchIndex);
