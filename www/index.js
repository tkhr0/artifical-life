import { Universe } from 'artifical-life'

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

requestAnimationFrame(renderLoop)
