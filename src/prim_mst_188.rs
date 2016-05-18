use std::thread;
use std::sync::{Arc, Mutex};
use std::io;
use std::io::{BufReader,BufRead,Read,stdin};
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::collections::VecDeque;
use std::f32;

use std::env;
//extern crate time;
//use time::PreciseTime;
use std::fmt::{ Display, Formatter, Error };

static mut dist: [i32; 500] = [9999999; 500];
static mut marked: [i32; 500] = [0; 500];
static mut per: [i32; 500] = [0; 500];

static mut adj: [[i32; 500]; 500] = [[0; 500]; 500];
pub fn prim_mst(matr: &mut [[i32; 500]; 500], n: i32) {
unsafe{
  	for i in 0..n {
      	for j in 0..n {
      		adj[i as usize][j as usize] = matr[i as usize][j as usize];
      	}
    }

    let (tx, rx) = mpsc::channel();
    let mut Vt = Vec::new();
    
    let mut cst = 0 ;
    let mut val : i32 = 0;
    let mut ver : i32= 0;
    let mut tper : i32 = 0;
    let mut nt : f32 = n as f32;
    let mut start: i32 = 0;

    let mut nm: i32 = nt.sqrt().ceil().abs() as i32;
    let mut thrd : i32 = nt.sqrt().floor().abs() as i32;
    //println!("{:?} and {:?}", nm, thrd );

    

    

//    let mut it:i32 = 0;
  dist[start as usize] = 0;
  per[start as usize] = 0;
  //let start = PreciseTime::now();

  while Vt.len() != n as usize {

    Vt.push(ver);
    marked[ver as usize] = 1;
    tper = per[ver as usize];
    cst += adj[ver as usize][tper as usize];
    println!(" Node b/w {:?} and {:?} and value {:?}", ver , tper, adj[ver as usize][tper as usize] );


    for x in 0..nm{

      let tx = tx.clone();
      let mut srt:i32 = x * thrd;
      let mut fns: i32= x * thrd + thrd;
      
      if fns > n{

          fns = n;
      }
      //println!("{:?} and {:?} and", srt , fns );
      thread::spawn(move ||{

        let mut value : i32 = 9999999;
        let mut Vsend :i32 =  0;
        for i in srt..fns{
          

          if marked[i as usize] == 0{
      
              if adj[ver as usize ][i as usize ] < dist[i as usize ] && adj[ver as usize][i as usize] != 0 {

                dist[i as usize] = adj[ver as usize][i as usize];
                per[i as usize] = ver;
            
              }

              if dist[i as usize] < value{
              value = dist[i as usize];
              Vsend = i;
              }
          }

      }
      tx.send(Vsend).unwrap();
      //println!("thread {} finished", x);
      });

    }
    let mut v : i32 = 0;
    val =  999999;
    for _ in 0..nm{

       v = rx.recv().unwrap();
    
      if dist[v as usize] < val && v != 0 {

        val = dist[v as usize];
        ver = v;
      }
            //println!("{}", v);
      }
      //println!("{:?}", dist);
      //for i in 0..n {
     //   print!("{} ", dist[i as usize]);
     // }

  }

    //let end = PreciseTime::now();
   // println!("{}  seconds for whatever you did.", start.to(end));

      println!(" total cost {}", cst);     
  }
}
