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

legend water #2ebdbd
legend sand #ceae7f
legend grass #33480d
legend rock_lower #41474a
legend rock_upper #504543

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
  `;

  constructor(private compiler: CompilerService) {
  }

  ngOnInit() {}

  convertColor(color: string) {
    const r = color.substring(0, 2);
    const g = color.substring(2, 4);
    const b = color.substring(4, 6);
    // parse hex to int
    return [parseInt(r, 16), parseInt(g, 16), parseInt(b, 16)];
  }

  ngAfterViewInit() {
    const canvas = this.canvas.nativeElement;
    const ctx = canvas.getContext('2d');
    if (!ctx) {
      console.log('aaa no ctx', ctx);
      return;
    }
    this.ctx = ctx;
    this.run();
  }

  compile(program: string) {
    const tokens = this.compiler.tokenize(program);
    const world = this.compiler.parse(tokens);
    return world;
  }

  generate(world: World) {
    console.log('aaa');
    const canvas = this.canvas.nativeElement;
    const ctx = this.ctx;
    const image = ctx.createImageData(canvas.width, canvas.height);
    let data = image.data;

    let regionPerc = 0;
    const regions = world.regions.map(region => {
      const previousRegionPerc = regionPerc;
      regionPerc += region.percent / 100;
      return previousRegionPerc;
    });
    console.log(regions);
    // 0, 0.5

    const subregions = world.regions.map((region, index) => {
      const start = regions[index];
      let end = regions[index + 1];
      if (index == regions.length - 1) {
        end = 1;
      }
      const diff = end - start;

      let subregionPerc = 0;
      let subregions = region.subRegions.map(subregion => {
        const previousSubregionPerc = subregionPerc;
        subregionPerc += subregion.percent / 100;
        return previousSubregionPerc;
      });

      subregions = subregions.map(subregion => {
        return subregion * diff + start;
      });

      console.log(subregions);
      return subregions;
    });

    for (let i = 0; i < canvas.width; i++) {
      for (let j = 0; j < canvas.height; j++) {
        const x = (i / canvas.width) * 4;
        const y = (j / canvas.height) * 4;
        const noise = (this.noise.gen(x, y) + 1) / 2;

        let regionIndex = 0;
        for (; regionIndex < regions.length; ++regionIndex) {
          if (regions[regionIndex] > noise) {
            break;
          }
        }
        regionIndex--;

        // console.log("selection", selection);
        // console.log("regionIndex", regionIndex);

        const region = world.regions[regionIndex];
        const subregions_ = subregions[regionIndex];

        let subregionIndex = 0;
        for (; subregionIndex < subregions_.length; ++subregionIndex) {
          if (subregions_[subregionIndex] > noise) {
            break;
          }
        }
        subregionIndex--;

        // console.log("noise", noise);
        // console.log("subregionindex", subregionIndex);

        const subregion = region.subRegions[subregionIndex];

        const [r, g, b] = this.convertColor(subregion.color);
        const index = (j * canvas.width + i) * 4;
        data[index] = r;
        data[index + 1] = g;
        data[index + 2] = b;
        data[index + 3] = 255;
      }
    }

    ctx.putImageData(image, 0, 0);
  }

  editorOptions = { theme: 'vs-dark', language: 'text' };

  run() {
    this.noise = new tumult.Simplex2();

    const [world, error] = this.compile(this.code);

    if (error) {
      console.log('Error go brr', error);
    }

    console.log(world);
    this.generate(world);
  }
}
