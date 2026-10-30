namespace $.$$ {

	type $bog_rsdemo_task = { id : number , title : string , done : boolean }
	type $bog_rsdemo_state = { board : $bog_rsdemo_task[] , shelf : $bog_rsdemo_task[] , summary : string }

	export class $bog_rsdemo extends $.$bog_rsdemo {

		@ $mol_mem
		rs() {
			const buffer = new Uint8Array( $mol_fetch.buffer( 'web.wasm' ) )
			const rs = new $mol_wire_rs( buffer , {
				bog_rsdemo: {
					render: ( ptr : number , len : number )=> this.state( JSON.parse( rs.string( ptr , len ) ) ),
				},
			} )
			rs.call( 'bog_rsdemo_start' )
			return rs
		}

		@ $mol_mem
		state( next? : $bog_rsdemo_state ) {
			return next ?? { board : [] , shelf : [] , summary : '' }
		}

		auto() {
			this.rs()
		}

		@ $mol_mem
		tasks() {
			const { board , shelf } = this.state()
			return new Map( [ ... board , ... shelf ].map( task => [ task.id , task ] ) )
		}

		board() {
			return this.state().board.map( task => this.Task( task.id ) )
		}

		shelf() {
			return this.state().shelf.map( task => this.Archived( task.id ) )
		}

		summary() {
			return this.state().summary
		}

		alive_label() {
			this.state()
			return `nodes: ${ this.rs().alive() }`
		}

		task_title( id : number ) {
			return this.tasks().get( id )?.title ?? ''
		}

		task_done( id : number , next? : boolean ) {
			if( next !== undefined ) {
				this.rs().call( 'bog_rsdemo_done' , id , next ? 1 : 0 )
				return next
			}
			return this.tasks().get( id )?.done ?? false
		}

		add() {
			const title = this.draft().trim()
			if( !title ) return
			this.rs().call( 'bog_rsdemo_add' , ... this.rs().text( title ) )
			this.draft( '' )
		}

		shelve( id : number ) {
			this.rs().call( 'bog_rsdemo_shelve' , id )
		}

		remove( id : number ) {
			this.rs().call( 'bog_rsdemo_remove' , id )
		}

	}

}
