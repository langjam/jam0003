import { Component, ElementRef, ViewChild } from '@angular/core';

import * as tumult from 'tumult';
import { CompilerService, World } from './services/compiler.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
})
export class AppComponent {
  title = 'webapp';
  noise: any;

  @ViewChild('canvas') canvas!: ElementRef<HTMLCanvasElement>;

  ctx!: CanvasRenderingContext2D;

  code: string = `
world hello

legend water #0000ff
legend sand #ff0000
legend grass #00ff00
legend rock_lower #899012
legend rock_upper #999999

region island
island 50%
island 7x7  // max size
island water 25%
island sand 25%
island grass 25%

region rocks
rocks 50%
rocks 5x5
rocks rock_upper 50%
rocks rock_lower 50%
  `

  constructor(private compiler: CompilerService) {
    this.noise = new tumult.Simplex2();
  }

  ngOnInit() {
  }

  ngAfterViewInit() {
    const canvas = this.canvas.nativeElement;
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      console.log("aaa no ctx", ctx);
      return;
    }
    this.ctx = ctx;

    const [world, error] = this.compile(this.code);

    if (error) {
      console.log("Error go brr", error);
    }

    this.generate(world);
  }

  compile(program: string) {
    const tokens = this.compiler.tokenize(program);
    console.log(tokens);
    const world = this.compiler.parse(tokens);
    return world;
  }

  generate(world: World) {
    console.log("aaa");
    const canvas = this.canvas.nativeElement;
    const ctx = this.ctx;
    const image = ctx.createImageData(canvas.width, canvas.height);
    let data = image.data;

    for (let i = 0; i < canvas.width; i++) {
      for (let j = 0; j < canvas.height; j++) {
        const x = (i / canvas.width) * 4;
        const y = (j / canvas.height) * 4;
        const noise = (this.noise.gen(x, y) + 1) / 2;
        const index = (j * canvas.width + i) * 4;
        data[index] = noise * 255;
        data[index + 1] = noise * 255;
        data[index + 2] = noise * 255;
        data[index + 3] = 255;
      }
    }

    ctx.putImageData(image, 0, 0);
  }

  editorOptions = { theme: 'vs-dark', language: 'javascript' };
}
