import { Universe, Life } from 'artifical-life'
import { memory } from 'artifical-life/artifical_life_bg'

const FIELD_COLOR = '#000'

const universe = Universe.new()
const width = universe.width()
const height = universe.height()

const canvas = document.getElementById('canvas-universe')
const ctx = canvas.getContext('2d')

canvas.width = width * 1.2
canvas.height = height * 1.2

const renderLoop = () => {
  drawField()
  drewLives()

  requestAnimationFrame(renderLoop)
}

const drawField = () => {
  ctx.beginPath()
  ctx.strokeStyle = FIELD_COLOR

  ctx.moveTo(0, 0)
  ctx.lineTo(width, 0)
  ctx.lineTo(width, height)
  ctx.lineTo(0, height)
  ctx.closePath()

  ctx.stroke()
}

const drewLives = () => {
  const livesPtr = universe.lives()
  const lives = new Uint8Array(memory.buffer, livesPtr, universe.lives_size())

  console.log('lives: ', lives)

  ctx.beginPath()
  for(let i=0; i < universe.lives_size(); i++) {
    const life = lives[i]
    ctx.arc(life.x, life.y, 5, 0, Math.PI * 2, true)
  }
  ctx.stroke()

  ctx.beginPath()
    ctx.arc(50, 50, 10, 0, Math.PI * 2, true)
  ctx.stroke()
}

drawField()
drewLives()
requestAnimationFrame(renderLoop)
