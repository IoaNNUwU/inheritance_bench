#include <vector>
#include <memory>

#include <iostream>
#include <chrono>
#include <ctime> 

typedef unsigned int u32;
typedef float f32;

using namespace std;

struct Color {
    u32 c;

    Color(u32 c) {
        this->c = c;
    }
    Color() {
        this->c = 0xFF000000;
    }
};

struct Point {
    f32 x;
    f32 y;
};

struct Screen {
    template<typename T>
    void fill_whatever(T whatever) {
        cout << whatever << endl;
    }
};

struct Draw {
    virtual void draw(Screen* screen) {};
};

struct Square: Draw {
    Point pos;
    f32 w;
    Color color;

    Square(Point pos, f32 w, Color color) {
        this->pos = pos;
        this->w = w;
        this->color = color;
    }
    
    virtual void draw(Screen* screen) {
        screen->fill_whatever(this);
    }

    f32 area() {
        return w * w;
    }
};

struct Rectangle: Square {
    f32 h;

    Rectangle(Point pos, f32 w, f32 h, Color color): Square(pos, w, color) {
        this->h = h;
    }

    f32 area() {
        return w * h;
    }
    
    virtual void draw(Screen* screen) {
        screen->fill_whatever(this);
    }
};

struct Triangle: Draw {
    Point p1;
    Point p2;
    Point p3;
    Color color;

    Triangle(Point p1, Point p2, Point p3, Color color) {
        this->p1 = p1;
        this->p2 = p2;
        this->p3 = p3;
        this->color = color;
    }
    
    virtual void draw(Screen* screen) {
        screen->fill_whatever(this);
    }
};

struct Circle: Draw {
    Point pos;
    f32 radius;
    Color color;

    Circle(Point pos, f32 radius, Color color) {
        this->pos = pos;
        this->radius = radius;
        this->color = color;
    }

    f32 area() {
        return radius * radius * 3.1416;
    }

    virtual void draw(Screen* screen) {
        screen->fill_whatever(this);
    }
};

struct Star: public Circle {
    f32 angle;

    Star(Point pos, f32 radius, f32 angle, Color color): Circle(pos, radius, color) {
        this->angle = angle;
    }
    
    virtual void draw(Screen* screen) {
        screen->fill_whatever(this);
    }
};

class Graphics {
    vector<unique_ptr<Draw>> v;

public:
    void add(unique_ptr<Draw> comp) {
        v.push_back(move(comp));
    }

    void draw(Screen* screen) {
        for (auto const& comp : v) {
            comp->draw(screen);
        }
    };
};

int main() {
    Screen screen;
    Graphics graphics;

    for (int i = 0; i < 100; i++) {
        graphics.add(make_unique<Square>(Square {
            Point { 10 + (float)i, 11 - (float)i },
            10 + (float)i,
            Color { 0xFF000000 }
        }));
        
        graphics.add(make_unique<Rectangle>(Rectangle {
            Point { 1 + (float)i, 39 - (float)i },
            10 + (float)i,
            10 - (float)i,
            Color { 0xFF000000 }
        }));
        graphics.add(make_unique<Circle>(Circle {
            Point { 14 + (float)i, 22 - (float)i },
            8 * (float) i,
            Color { 0xFF000000 }
        }));
        
    }
    
    auto start = std::chrono::system_clock::now();

    graphics.draw(&screen);

    auto end = std::chrono::system_clock::now();
 
    std::chrono::duration<double> elapsed_seconds = end-start;
    std::time_t end_time = std::chrono::system_clock::to_time_t(end);
 
    std::cout << "finished computation at " << std::ctime(&end_time)
              << "elapsed time: " << elapsed_seconds.count() << "s"
              << std::endl;

}