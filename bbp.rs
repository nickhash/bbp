/**
	Author: Nick <afterether@gmail.com>

	Bread Buying Problem
	Input:
		
		- total_days, an integer, the number of days in the calendar until the free bread arrives
		- list of quantity-price pairs, separated by SPACE, like:
			[(10,200) (15,100) (35,500) (50,30)]) -> [5,30,5,10]

	Output:

		Purchase plan that minimizes the total cost

	Restrictions:
		- Do not use any dependencies

	Documentation: 
		located at the repo
*/
use std::{
	env,
	cmp::Ordering,
	fmt,
};

macro_rules! fatal {
	($msg:expr) => {
		println!("{}", $msg); 
		std::process::exit(1);
	};
}

const BREAD_EXPIRATION: u32 = 30;
const INITIAL_BREAD: u32 = 10;

type Day = u32;
type Qty = u32;
type Price = u32;
type ParsedEvents = Vec<SellEvent>;
type Purchases = Vec<Option<Qty>>;
type Availability = Vec<bool>;
type Calendar = Vec<Availability>;

/// Quantity-Price pair (something like the concept of Key-Value pair)
#[derive(Debug, Clone, PartialEq, Eq)]
struct QPPair {
	qty:		Qty,
	price:		Price,
}
impl QPPair {
}
/// Event where the User can buy bread, it is an entry in the Calendar
#[derive(Debug, Clone, PartialEq, Eq)]
struct SellEvent {
	day:		Day,
	price:		Price,
}
impl SellEvent {
	pub fn new(in_evt_str: &str) -> Self {
		if in_evt_str.len() < 5 {
			// ([:digit],[:digit:]) format is a must, so min len() is 5
			fatal!(format!("Invalid entry length for: {} (must be >= 5 chars",in_evt_str));
		}
		let evt_str: String = in_evt_str[1..(in_evt_str.len()-1)].to_owned();
		let mut split = evt_str.split(",");
		let day = match split.next() {
			Some(day_str) => {
				let d = match day_str.parse::<Day>() {
					Ok(d) => d,
					Err(err) => {fatal!(format!("Error parsing calendar days ({}) : {}",day_str,err));},
				};
				if d == 0 {
					fatal!(format!("Day can't be 0: {}",evt_str));
				}
				d
			},
			None => {
				fatal!(format!("Entry erroneous and can't be parsed: {}",evt_str));
			},
		};
		let price = match split.next() {
			Some(price_str) => {
				let p = match price_str.parse::<Price>() {
					Ok(p) => {
						if p == 0 {
							fatal!(format!("Price can't be 0: {}",evt_str));
						}
						p
					},
					Err(err) => {
						fatal!(format!("Error parsing price ({}): {}",price_str,err));
					},
				};
				p
			},
			None => {
				fatal!(format!("Entry for Sell Event is invalid: {}",evt_str));
			},
		};
		SellEvent {
			day,
			price,
		}
	}
}
/// Concentrates multiple Sell Events per day, to facilitate the process
#[derive(Debug, Clone, PartialEq, Eq)]
struct EventList {
	day:		Day,
	events:		Vec<SellEvent>,
}
impl EventList {
	pub fn new(day: Day) -> Self {
		EventList {
			day,
			events: Vec::new(),
		}
	}
	pub fn add_event(&mut self,se: SellEvent) {
		self.events.push(se);
	}
}
impl fmt::Display for EventList {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut output = format!("Day: {} has {} events:\n",self.day,self.events.len());
		for e in &self.events {
			output.push_str(&format!("\tday: {: >3}\t{: >5}\n",e.day,e.price));
		}
		f.write_str(&output)
	}
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Providers(Vec<Price>);
impl Providers {
	pub fn new() -> Self {
		Providers(Vec::new())
	}
	pub fn add_provider(&mut self,price: Price) -> usize {
		if self.find_provider(price).is_some() {
			fatal!(format!("Provide with price {} already registered, invalid input",price));
		}
		let idx=self.0.len();
		self.0.push(price);
		idx
	}
	pub fn find_provider(&self,price: Price) -> Option<usize> {
		let mut idx = 0usize;
		for p in &self.0 {
			if *p == price {
				return Some(idx);
			}
			idx += 1;
		}
		None
	}
	pub fn sort_by_price(&self) -> Vec<usize> {

		let mut pairs: Vec<(usize,Price)> = self.0.iter().enumerate().map(|(k,vptr)|(k,*vptr)).collect();
		pairs.sort_by(|(_k1,v1),(_k2,v2)|v1.cmp(v2));
		let sorted: Vec<usize> = pairs
			.iter()
			.enumerate()
			.map(|(_k,v)|{v.0})
			.collect();
		sorted
	}
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Environment {
	event_list:		Vec<EventList>,
	providers: 		Providers,
	avail_matrix:	Calendar,
}
impl Environment {
	pub fn new() -> Self {
		Environment {
			event_list:		Vec::new(),
			providers:		Providers::new(),
			avail_matrix:	Vec::new(),
		}
	}
}
fn parse_input(args: Vec<String>) -> (Day,ParsedEvents) {
	// Return values:
	//		Length of the calendar in Day(s)

	if args.len() != 3 {
		fatal!(
			format!(
				"Usage:\n\t{} [num_days] [list of quantity-price pairs]\n\n\
				Example:\n\t{} 60 \"(15,100) (35,500) (50,30)\"\n\n",
				args[0],
				args[0]
			)
		);
	}
	let days = match args[1].parse::<u32>() {
		Ok(d) => {
			if d == 0 {
				fatal!(
					format!("Day number is 0 : {}",args[1])
				);
			}
			d
		},
		Err(err) => {
			fatal!(
				format!("Couldn't parse number of days in the calendar ({}) : {}",args[1],err)
			);
		},
	};
	let mut prev_day: Option<Day> = None;
	let mut events: ParsedEvents = Vec::new();
	let parsed = args[2].split(" ");
	for s in parsed {
		let sell_event = SellEvent::new(s);
		if prev_day.is_some() {
			if *prev_day.as_ref().unwrap() > sell_event.day {
				fatal!(
					format!("List of purchases doesn't have consecutive day number : {:?}",sell_event)
				);
			} else {
				prev_day = Some(sell_event.day);
			}
		}
		events.push(sell_event);
	}
	if events.is_empty() {
		fatal!("The calendar is empty");
	}
	(days,events)
}
fn make_environment(mut parsed_events: ParsedEvents) -> Environment {
	// If we use the terminology of Reinforcement Learning , then:
	//		Environment:  
	//			The calendar with available purchasin days is our Environment
	//			Environment = Vec<EventList>, where EventList = Vec<SellEvent> + aux stuff
	//		Actor: 
	//			is the user who is making purchasing actions
	let mut environment = Environment::new();

	parsed_events.sort_by(|e1,e2| {
		if e1.day > e2.day {
			Ordering::Greater
		} else {
			if e1.day < e2.day {
				Ordering::Less
			} else {
				// days are the same
				if e1.price > e2.price {
					Ordering::Greater
				} else {
					if e1.price < e2.price {
						Ordering::Less
					} else {
						Ordering::Equal
					}
				}
			}
		}
	});
	for e in &parsed_events {
		environment.providers.add_provider(e.price);
	}
	let mut daily_events= Vec::new();
	let num_entries = parsed_events.len();
	let mut cur_idx: usize = 0;
	let mut daily = EventList::new(parsed_events[cur_idx].day);
	// groups SellEvents per day
	while cur_idx < num_entries {
		daily.add_event(parsed_events[cur_idx].clone());
		cur_idx += 1;

		if cur_idx < num_entries {
			// cuts off previous day, and starts another
			if parsed_events[cur_idx].day != daily.day {
				daily_events.push(daily.clone());
				daily = EventList::new(parsed_events[cur_idx].day);
			}
		} else {
			// executes only after we reached the last element of the loop
			daily_events.push(daily.clone());
		}
	}
	environment.event_list = daily_events;
	environment
}
fn generate_empty_availability_vec(num_elts: usize) -> Availability {

	std::iter::repeat(false).take(num_elts).collect()
}
fn set_availability_at(a: &mut Availability,start_idx: usize,calendar_days: Day) {
	
	let mut limit = start_idx + BREAD_EXPIRATION as usize;
	if limit >= calendar_days as usize {
		limit = calendar_days as usize;
	}
	for i in start_idx..limit {
		a[i]=true;
	}
}
fn calculate_bread_availability(environment: &Environment,calendar_days: Day) -> Calendar {
	// expands possible purchases into availability of the bread according to expiration date
	// Availavilty of the bread is a Matrix of boolean values, where:
	//		true =  bread is still fresh
	//		false = bread is stale 
	
	let mut cal: Calendar = Vec::new();
	for _ in &environment.providers.0 {
		cal.push(generate_empty_availability_vec(calendar_days as usize));
	}

	for daily_events in &environment.event_list {
		for single_event in &daily_events.events {
			let pidx = environment.providers.find_provider(single_event.price).expect("Provider not found");
			set_availability_at(&mut cal[pidx],single_event.day as usize,calendar_days);
		}
	}
	cal
}
fn cheapest_bread_for_day(avail_matrix: &Calendar,prov_list: &Vec<usize>, day_num: Day) -> Option<usize> {
	// returns provider for the cheapest bread

	for pidx in prov_list.iter() {
		if *avail_matrix[*pidx].get(day_num as usize).unwrap() {
			return Some(*pidx);
		}
	}
	None	// this is returned when family must eat stale bread
}
fn solve(total_days: Day,environment: &Environment) -> Purchases {
	// Return Value:
	//		return the Vector of optimal purchases, where single purchase = Qty

	assert_ne!(environment.event_list.len(),0);

	let mut solution: Purchases = Vec::new();
	let mut day = 0;
	assert_eq!(total_days>day,true);

	let providers = environment.providers.sort_by_price();	
	let mut minimum_providers: Vec<Option<usize>> = Vec::new();

	// build the vector of indices of providers with minimal prices
	loop {
		let min_prov = cheapest_bread_for_day(&environment.avail_matrix,&providers,day);
		minimum_providers.push(min_prov);
		day += 1;
		if day == total_days {
			break;
		}
	}
	let mut mp_str = String::from("");
	for p in &minimum_providers {
		match &p {
			&Some(v) => mp_str.push_str(&format!("{},",v)),
			None => mp_str.push_str(&format!("None,")),
		}
	}
	// build solution vector
	let mut counter = 0usize;
	assert_eq!(counter>=minimum_providers.len(),false);
	let mut previous: Option<usize> = minimum_providers[0].clone();
	let mut qty_accum: i32 = 0-INITIAL_BREAD as i32;// negative qty to discount for existing inventory
	while counter < minimum_providers.len() {
		match &previous {
			&Some(prev_prov) => {
				match minimum_providers[counter] {
					Some(min_prov) => {
						if min_prov == prev_prov {
							// no change in provider possible, so we accumulate qty for the order
						} else {
							solution.push(Some(qty_accum as Qty));
							qty_accum = 0;	// now we are ready for a new order
						}
					},
					None => {
						solution.push(Some(qty_accum as Qty)); // make purchases for previously available bread
						solution.push(None);	// stale bread is eaten from here
					},
				}
				qty_accum += 1;
				previous = minimum_providers[counter].clone();
			},
			None => {
				if qty_accum < 0 {
					qty_accum += 1;			// consume initial bread inventory
				} else {
					match minimum_providers[counter] {
						Some(_) => {
							qty_accum=1;	// all initial inventory was eaten, so we start buying first loaf
						},
						None => {
							solution.push(None);	// stale bread is eaten
						},
					}
					previous = minimum_providers[counter].clone();
				}
			},
		}
		counter += 1;
	}
	match &previous {
		&Some(_) => solution.push(Some(qty_accum as Qty)),
		None => (),// we already pushed None in steps before
	}

	solution
}
#[allow(dead_code)]
fn dump_availability_matrix(c: &Calendar) {
	
	for provider in c {
		for a in provider {
			if *a {
				print!("1");
			} else {
				print!("0");
			}
		}
		println!("");
	}
}
fn print_purchasing_plan(purchases: &Purchases) {

	println!(
		"{}",
		purchases
			.iter()
			.map(|p|{if p.is_none() {"None".to_owned()} else {p.as_ref().unwrap().to_string()}})
			.collect::<Vec<String>>()
			.join(",")
	);
}
fn main() {

	let args: Vec<String> = env::args().collect();
	let (calendar_length,unordered_events) = parse_input(args);
	let mut environment = make_environment(unordered_events);
	environment.avail_matrix= calculate_bread_availability(&environment,calendar_length);
	//dump_availability_matrix(&environment.avail_matrix);
	let solution = solve(calendar_length,&environment);
	print_purchasing_plan(&solution);
}
