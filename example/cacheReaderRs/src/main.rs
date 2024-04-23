use std::collections::{HashMap, VecDeque};
use std::env;
mod lru_cache;
use lru_cache::LRUCache;
use cache_reader::{Request, ObjIdT, ReqOpE, KVdata, open_trace_oracle_rs, get_next_request_rs, close_trace_rs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <trace_path> <cache_size>", args[0]);
        return;
    }

    let path = &args[1];
    let cache_size_in_mb: usize = args[2].parse().expect("Cache size must be a number");
    let cache_size: usize = cache_size_in_mb * 1024 * 1024;
    let period = 10000;
    let mut lru_cache = LRUCache::new(cache_size);
    let mut size_record: HashMap<u64, usize> = HashMap::new();

    let reader_idx = open_trace_oracle_rs(path);
    if reader_idx < 0 {
        println!("Failed to open trace: {}", path);
        return;
    }

    // let mut cache_map: HashMap<u64, usize> = HashMap::new();
    // let mut lru_deque: VecDeque<u64> = VecDeque::new();
    let mut total_requests = 0;
    let mut cache_hits = 0;
    let mut cache_bandwidth_used = 0;
    // let mut current_cache_size = 0;
    let mut last_report_time: u64 = 0;
    let mut total_time_span: u64 = 0;
    let mut total_bw_used: usize = 0;

    loop {
        let req = get_next_request_rs(reader_idx);
        let mut curr_time = 0;
        match req {
            Some(req) if req.valid != 0 => {
                total_requests += 1;
                curr_time = req.clock_time;
                let found_size = lru_cache.get(req.obj_id);
                match found_size {
                    Some(_size) => {
                        cache_hits += 1;
                    },
                    None => {
                        lru_cache.put(req.obj_id, req.obj_size as usize);
                        cache_bandwidth_used += req.obj_size as usize;
                    }
                }
                // process_request(&req, &mut cache_map, &mut lru_deque, &mut current_cache_size, cache_size, &mut cache_hits, &mut cache_bandwidth_used);
                if size_record.get(&req.obj_id).is_some() {
                    size_record.remove(&req.obj_id);
                }
                size_record.insert(req.obj_id, req.obj_size as usize);
            },
            None => {
                // println!("No more requests in the trace");
                break;
            },
            _ => {}
        }

        if curr_time > last_report_time + period {
            let hit_ratio = cache_hits as f64 / total_requests as f64;
            let time_span = curr_time - last_report_time;
            total_time_span += time_span;
            total_bw_used += cache_bandwidth_used;
            last_report_time = curr_time;
            println!("Total requests: {}, Miss ratio: {:.2}, Bandwidth used: {:.3} Mbps",
                     total_requests, 1. - hit_ratio, 8. * cache_bandwidth_used as f64 / 1024. / 1024. / time_span as f64);
            cache_bandwidth_used = 0;
        }
        // if total_requests > 10000 {
        //     break;
        // }
    }
    close_trace_rs(reader_idx);
    // compute total workingset size
    let mut total_workingset_size = 0;
    for (_, size) in size_record.iter() {
        total_workingset_size += size;
    }
    // print out final stats
    let hit_ratio = cache_hits as f64 / total_requests as f64;
    if total_time_span != 0 {
        println!("======\nTotal requests: {}, Miss ratio: {:.2}, Total data: {} MB or {:.3} Mbps",
                total_requests, 1. - hit_ratio, total_bw_used / 1024 / 1024,
                8. * total_bw_used as f64 / 1024. / 1024. / total_time_span as f64
            );
    } else {
        println!("======\nTotal requests: {}, Miss ratio: {:.2}, Total data: {} MB. Total time span is zero, so Mbps cannot be calculated.",
            total_requests, 1. - hit_ratio, total_bw_used / 1024 / 1024
        );
    }
    println!("== workingset size: {} MB ==", total_workingset_size / 1024 / 1024);
}

// fn process_request(req: &Request, cache_map: &mut HashMap<u64, usize>, lru_deque: &mut VecDeque<u64>, current_cache_size: &mut usize, cache_size: usize, cache_hits: &mut u32, cache_bandwidth_used: &mut usize) {
//     let obj_id = req.obj_id;
//     let obj_size = req.obj_size as usize;

//     if let Some(&size) = cache_map.get(&obj_id) {
//         *cache_hits += 1;
//         *cache_bandwidth_used += size;
//         update_lru(obj_id, lru_deque);
//     } else {
//         make_room_for_new_item(obj_size, cache_map, lru_deque, current_cache_size, cache_size);
//         cache_map.insert(obj_id, obj_size);
//         lru_deque.push_front(obj_id);
//         *current_cache_size += obj_size;
//     }
// }

// fn make_room_for_new_item(new_size: usize, cache_map: &mut HashMap<u64, usize>, lru_deque: &mut VecDeque<u64>, current_cache_size: &mut usize, cache_size: usize) {
//     while *current_cache_size + new_size > cache_size {
//         if let Some(evict_id) = lru_deque.pop_back() {
//             if let Some(evict_size) = cache_map.remove(&evict_id) {
//                 *current_cache_size -= evict_size;
//             }
//         } else {
//             break;
//         }
//     }
// }

// fn update_lru(obj_id: u64, lru_deque: &mut VecDeque<u64>) {
//     if let Some(position) = lru_deque.iter().position(|&x| x == obj_id) {
//         lru_deque.remove(position);
//     }
//     lru_deque.push_front(obj_id);
// }
