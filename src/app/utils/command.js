import 'fs'
import 'handler.js'

class Command {
  constructor () {}

  exec () {}
}

class InputCommand extends Command {
  constructor () {}

  exec () {}

  input () {}
}

class InputHandler extends Command {
  constructor () {}

  exec () {
    this.handle()
  }

  handle () {
    this.#inputCommand.input()
  }

  #command = new Command()
  #inputCommand = new InputCommand()
}
